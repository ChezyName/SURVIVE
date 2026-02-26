from pathlib import Path
import os

def CountLines(file_path):
    with open(file_path, 'r', encoding='utf-8') as file:
        return sum(1 for line in file if (l := line.strip()) and not l.startswith('//'))
    
def CountAllLines():
    directory_path = Path("src")
    Files = list(directory_path.glob('**/*'))
    
    total = 0
    dir_totals = {}
    max_length = max(len(os.path.basename(str(f))) for f in Files if f.is_file())

    for filePath in Files:
        if not filePath.is_file():
            continue
        lines = CountLines(str(filePath))
        file_name = os.path.basename(str(filePath))
        dir_name = str(filePath.parent)
        
        print(f"{file_name.ljust(max_length)} | {lines} Lines")
        
        total += lines
        dir_totals[dir_name] = dir_totals.get(dir_name, 0) + lines

    print("\n\n")
    print(f"{'Total'.ljust(max_length)} | {total} Lines")
    print(f"{''.ljust(max_length)} |")
    for dir_name, dir_total in sorted(dir_totals.items()):
        print(f"{dir_name.ljust(max_length)} | {dir_total} Lines")

if __name__ == "__main__":
    CountAllLines()