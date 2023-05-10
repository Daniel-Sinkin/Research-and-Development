import re
import requests
import sys

def get_exercise_md(url):
    response = requests.get(url)
    if response.status_code != 200:
        raise Exception(f"Error downloading file: {response.status_code}")

    md_text = response.text

    # Find the exercises section and extract the text
    exercises_pattern = r'## Exercises\n\n(.*?)\n\n' # matches the section between the newline after '## Exercises' and the newline before "Remainder"
    exercises_text = re.search(exercises_pattern, md_text, flags=re.DOTALL)

    if not exercises_text:
        raise Exception("Exercises section not found")

    return exercises_text.group(1)

def exerciseMD_to_Latex(md_text):
    # Replace numbered list items
    md_text = re.sub(r'^\s*\d+\.', r'\n\\item', md_text, flags=re.MULTILINE)

    # Replace nested list items
    md_text = re.sub(r'^\s{4}\d+\.', r'\n    \\item', md_text, flags=re.MULTILINE)

    # Add enumerate environment for main list
    md_text = "\\begin{enumerate}\n" + md_text + "\n\\end{enumerate}"

    # Add nested enumerate environment
    md_text = re.sub(r'(\\item.*?)(\n    \\item)', r'\1\n  \\begin{enumerate}\2', md_text, flags=re.DOTALL)
    md_text = re.sub(r'(\n    \\item.*?)(\\item|\n\\end{enumerate})', r'\1\n  \\end{enumerate}\2', md_text, flags=re.DOTALL)

    return md_text

def processURL(url):
    latex_text = exerciseMD_to_Latex(get_exercise_md(url))
    
    # Extract the filename from the URL
    filename = url.split('/')[-1].split('.md')[0] + '.txt'
    
    with open(filename, 'w') as file:
        file.write(latex_text)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 exerciseRipper.py <url>")
        sys.exit(1)
    url = sys.argv[1]
    processURL(url)

