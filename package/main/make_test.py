import os
import pathlib

paths = ["Array", "Math", "Simple"]
for path in paths:
    path = f"src/{path}"
    files = os.listdir(path)
    files_file = [f for f in files if os.path.isfile(os.path.join(path, f))]
    for i in range(len(files_file)):
        print(files_file[i])
    for file in files_file:
        if file == "index.ts" or file == ".DS_Store" or file in "random":
            continue
        path = pathlib.Path(path)
        p_dir = path.name
        p_dir = f"tests/{p_dir}/"
        file = pathlib.Path(file).stem
        if not os.path.exists(p_dir):
            os.makedirs(p_dir)
        if os.path.isfile(p_dir + file + ".test.ts"):
            continue
        pathlib.Path(p_dir + file + ".test.ts").touch()
        with open(p_dir + file + ".test.ts", "w") as f:
            f.write(
                """import { %s } from "../../module/%s/%s";
test('{%s}', () => {});
"""
                % (file, path.name, file, file)
            )
