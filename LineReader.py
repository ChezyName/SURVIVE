#read how many lines
from pathlib import Path
import os

def CountLines(file_path):
    with open(file_path, 'r', encoding='utf-8') as file:
        return sum(1 for _ in file)
    
def CountAllLines():
    directory_path = Path("src")
    Files = list(directory_path.glob('**/*'))
    total = 0
    max_length = max(len(str(filePath)) for filePath in Files)  # Find the longest file path

    for filePath in Files:
        if(not filePath.is_file()):
           #print(str(filePath) + " is not file")
           continue
        lines = CountLines(str(filePath))
        file_name = os.path.basename(str(filePath))
        print(f"{file_name.ljust(max_length)} | {lines} Lines")  # Left-pad file paths
        total += lines

    print(f"{' ' * (max_length + 1)}| {total} Total Lines")  # Align the total count
    

if __name__ == "__main__":
    CountAllLines()