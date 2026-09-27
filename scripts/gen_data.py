import os, json, re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
chapters = []

for chapter_dir in sorted(os.listdir(ROOT)):
    full_chapter = os.path.join(ROOT, chapter_dir)
    if not os.path.isdir(full_chapter) or chapter_dir.startswith('.') or chapter_dir in ('docs',):
        continue
    if not re.match(r'^\d\d\.', chapter_dir):
        continue
    sections = []
    for sub_dir in sorted(os.listdir(full_chapter)):
        full_sub = os.path.join(full_chapter, sub_dir)
        if not os.path.isdir(full_sub):
            continue
        problem_path = os.path.join(full_sub, "problem.rs")
        answer_path = os.path.join(full_sub, "answer.rs")
        # cargo 프로젝트로 구성된 폴더(예: 5.5)는 src/bin/ 아래에 있음
        if not os.path.exists(problem_path):
            alt_problem = os.path.join(full_sub, "src", "bin", "problem.rs")
            alt_answer = os.path.join(full_sub, "src", "bin", "answer.rs")
            if os.path.exists(alt_problem):
                problem_path, answer_path = alt_problem, alt_answer
        section = {"title": sub_dir, "type": "code", "problem": "", "answer": "", "note": ""}
        if os.path.exists(problem_path):
            with open(problem_path, encoding="utf-8") as f:
                section["problem"] = f.read()
            with open(answer_path, encoding="utf-8") as f:
                section["answer"] = f.read()
        else:
            section["type"] = "note"
        # 코드 문제여도 개념설명.md 등이 같이 있으면 note로 같이 담는다 (7장 부록처럼)
        md_files = [f for f in os.listdir(full_sub) if f.endswith('.md')]
        if md_files:
            with open(os.path.join(full_sub, md_files[0]), encoding="utf-8") as f:
                section["note"] = f.read()
        sections.append(section)
    chapters.append({"title": chapter_dir, "sections": sections})

with open("" + os.path.join(ROOT, 'docs', 'data.js') + "", "w", encoding="utf-8") as f:
    f.write("const RUST_DRILLS_DATA = ")
    json.dump(chapters, f, ensure_ascii=False, indent=2)
    f.write(";\n")

print("chapters:", len(chapters))
for c in chapters:
    print(" -", c["title"], "sections:", len(c["sections"]))
