import json
import subprocess
import sys
import time
import pyperclip
from pathlib import Path
from selenium import webdriver
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.support.wait import WebDriverWait

base = Path(__file__).parent
(temp := base / "temp").mkdir(exist_ok=True)
save_path = base / "tags.json"

boj_url = "https://www.acmicpc.net"
num = sys.argv[1]


def get_tag_from_web():
    port = 9222

    subprocess.run(
        [
            "pwsh",
            "-c",
            f"chrome --remote-debugging-port={port} --user-data-dir={temp}",
        ],
        cwd=base,
    )

    options = webdriver.ChromeOptions()
    options.add_experimental_option("debuggerAddress", f"127.0.0.1:{port}")
    driver = webdriver.Chrome(
        service=Service(ChromeDriverManager().install()), options=options
    )

    try:
        login(driver)

        driver.get(f"{boj_url}/problem/{num}")
        tags = []

        try:
            tag_list = WebDriverWait(driver, 10).until(
                EC.presence_of_element_located((By.CSS_SELECTOR, ".spoiler-list"))
            )

            tags = [
                tag.text.strip() for tag in tag_list.find_elements(By.CSS_SELECTOR, "a")
            ]
        except:
            ...

        print(num, tags)
        save_tags(tags)

    finally:
        driver.quit()
        input("Done")
        subprocess.run(["pwsh", "-c", f"rm -r -force {temp}"])


def login(driver):
    ID, PASSWORD = (base / "boj_account.txt").read_text().splitlines()

    driver.get(f"{boj_url}/login")
    form = WebDriverWait(driver, 10).until(
        EC.presence_of_element_located((By.CSS_SELECTOR, "#login_form"))
    )

    id_input, pass_input = form.find_elements(By.CSS_SELECTOR, "input[placeholder]")
    id_input.send_keys(ID)
    time.sleep(0.3)
    pass_input.send_keys(PASSWORD)
    time.sleep(0.5)
    pass_input.send_keys(Keys.RETURN)

    WebDriverWait(driver, 30).until(EC.url_to_be(boj_url + "/"))


def get_tag_from_clipboard():
    tags = [tag.strip() for tag in pyperclip.paste().splitlines()]
    save_tags(tags)


def save_tags(tags):
    export = json.load(open(save_path, encoding="utf-8"))
    export[num] = tags
    export = {
        key: val
        for key, val in sorted(
            [item for item in export.items()], key=lambda x: int(x[0])
        )
    }

    json.dump(
        export,
        open(save_path, "w", encoding="utf-8"),
        ensure_ascii=False,
    )

    subprocess.run(["pwsh", "-c", f"prettier -w --print-width 1000 {save_path}"])


# get_tag_from_web()
get_tag_from_clipboard()
