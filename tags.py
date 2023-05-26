import time
import json
import subprocess
from pathlib import Path
from selenium import webdriver
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.support.wait import WebDriverWait

base = Path(__file__).parent

def get_tag():
    port = 9222
    subprocess.run(["chrome", f"--remote-debugging-port={port}", '--user-data-dir="/temp"'])
    input()

    options = webdriver.ChromeOptions()
    options.add_experimental_option("debuggerAddress", f"127.0.0.1:{port}")
    driver = webdriver.Chrome(service=Service(ChromeDriverManager().install()), options=options)

    save_path = base / "tags.json"
    export = json.load(open(save_path, encoding="utf-8"))
    max_num = max(map(int, export.keys()))
    print(max_num)
    
    try:
        for nums in [
            i
            for i in (base / "rust/src").iterdir()
            if i.is_dir()
        ]:
            for problem in nums.iterdir():
                num = int(problem.stem)

                if num <= max_num:
                    continue

                url = f"https://www.acmicpc.net/problem/{num}"
                driver.get(url)
                tags = []

                try:
                    tag_list = WebDriverWait(driver, 10).until(
                        EC.presence_of_element_located((By.CSS_SELECTOR, ".spoiler-list"))
                    )

                    for tag in tag_list.find_elements(By.CSS_SELECTOR, "a"):
                        tags.append(tag.text.strip())
                except:
                    ...

                print(num, tags)
                export[num] = tags
                time.sleep(1.5)
    finally:
        json.dump(
            export,
            open(save_path, "w", encoding="utf-8"),
            ensure_ascii=False,
            indent=4,
        )


get_tag()
