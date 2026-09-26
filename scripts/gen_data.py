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
        section = {"title": sub_dir, "type": "code", "problem": "", "answer": "", "note": ""}
        if os.path.exists(problem_path):
            with open(problem_path, encoding="utf-8") as f:
                section["problem"] = f.read()
            with open(answer_path, encoding="utf-8") as f:
                section["answer"] = f.read()
        else:
            # checklist or concept note (.md file)
            md_files = [f for f in os.listdir(full_sub) if f.endswith('.md')]
            section["type"] = "note"
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
