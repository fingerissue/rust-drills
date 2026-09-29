"""
grass 브랜치에 push된 커밋에서 변경된 problem.rs 파일들을 찾아
실제로 컴파일/실행해서 통과 여부를 판정하는 채점 스크립트.

- 일반 폴더(1~4장, 5장 대부분): rustc로 단일 파일 컴파일 후 실행
- Cargo 프로젝트 폴더(5.5, 06장): 해당 폴더의 Cargo.toml에서 이 파일과 매칭되는
  [[bin]] name을 찾아서 `cargo run --bin <name>` 으로 실행

main() 안의 assert_eq!/assert! 가 전부 통과하면 그 바이너리는 exit code 0으로 끝난다.
하나라도 실패하면(패닉) 0이 아닌 코드로 끝나므로, 그걸로 통과 여부를 판정한다.
"""
import os
import re
import subprocess
import sys


def get_changed_files() -> list[str]:
    before = os.environ.get("BEFORE_SHA", "")
    after = os.environ.get("AFTER_SHA", "HEAD")

    # -c core.quotePath=false: 한글 등 비-ASCII 파일명을 8진수 이스케이프+따옴표로 감싸지 않고
    # 그대로 출력하게 함. 이게 없으면 한글 경로의 problem.rs가 "...problem.rs" 처럼 끝에
    # 따옴표가 붙어서 나와, endswith("problem.rs") 검사가 전부 실패해 "변경 없음"으로
    # 잘못 처리되는(=채점 없이 항상 통과되는) 버그가 있었다.
    GIT = ["git", "-c", "core.quotePath=false"]

    def run_diff(a: str, b: str) -> str | None:
        r = subprocess.run(GIT + ["diff", "--name-only", a, b], capture_output=True, text=True)
        return r.stdout if r.returncode == 0 else None

    out = None
    if before and set(before) != {"0"}:
        out = run_diff(before, after)
    if out is None:
        # before가 없거나(첫 push) diff가 실패한 경우: 직전 커밋과 비교
        out = run_diff("HEAD~1", "HEAD")
    if out is None:
        # 그래도 안 되면 이번 커밋에 포함된 파일 전체
        r = subprocess.run(
            GIT + ["show", "--name-only", "--pretty=", "HEAD"], capture_output=True, text=True
        )
        out = r.stdout

    return [line.strip() for line in out.splitlines() if line.strip().endswith("problem.rs")]


def find_cargo_root(path: str) -> str | None:
    d = os.path.dirname(os.path.abspath(path))
    while d != "/":
        if os.path.exists(os.path.join(d, "Cargo.toml")):
            return d
        d = os.path.dirname(d)
    return None


def find_bin_name(cargo_root: str, rel_path: str) -> str | None:
    with open(os.path.join(cargo_root, "Cargo.toml"), encoding="utf-8") as f:
        content = f.read()
    for m in re.finditer(
        r'\[\[bin\]\]\s*\nname\s*=\s*"([^"]+)"\s*\npath\s*=\s*"([^"]+)"', content
    ):
        name, bin_path = m.group(1), m.group(2)
        if os.path.normpath(os.path.join(cargo_root, bin_path)) == os.path.normpath(
            os.path.join(cargo_root, rel_path)
        ):
            return name
    return None


def grade_standalone(path: str) -> bool:
    with open(path, encoding="utf-8") as f:
        source = f.read()
    is_test_file = "#[test]" in source

    binpath = "/tmp/graded_bin"
    args = ["rustc", "--edition", "2021"]
    if is_test_file:
        args.append("--test")
    args += [path, "-o", binpath]

    r = subprocess.run(args, capture_output=True, text=True)
    if r.returncode != 0:
        print(r.stdout)
        print(r.stderr)
        return False

    run_args = [binpath, "--include-ignored"] if is_test_file else [binpath]
    r2 = subprocess.run(run_args, capture_output=True, text=True)
    print(r2.stdout)
    if r2.returncode != 0:
        print(r2.stderr)
        return False
    return True


def grade_cargo(cargo_root: str, rel_path: str) -> bool:
    bin_name = find_bin_name(cargo_root, rel_path) or "problem"
    r = subprocess.run(
        ["cargo", "run", "--quiet", "--bin", bin_name], cwd=cargo_root, capture_output=True, text=True
    )
    print(r.stdout)
    if r.returncode != 0:
        print(r.stderr)
        return False
    return True


def grade_file(path: str) -> bool:
    print(f"\n===== 채점: {path} =====")
    cargo_root = find_cargo_root(path)
    if cargo_root is None:
        ok = grade_standalone(path)
    else:
        rel = os.path.relpath(path, cargo_root)
        ok = grade_cargo(cargo_root, rel)
    print("✅ 통과" if ok else "❌ 실패")
    return ok


def main() -> int:
    files = get_changed_files()
    if not files:
        print("변경된 problem.rs 없음 - 채점 스킵")
        return 0

    all_ok = True
    for f in files:
        if not os.path.exists(f):
            continue  # 삭제된 파일
        all_ok = grade_file(f) and all_ok

    print("\n========================================")
    print("🎉 전체 통과!" if all_ok else "😢 일부 실패 - 위 로그에서 어디서 막혔는지 확인")
    return 0 if all_ok else 1


if __name__ == "__main__":
    sys.exit(main())
