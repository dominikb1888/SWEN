# https://classroom.github.com/classrooms/98897122-22s_swen
import time
from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.common.by import By
from getpass import getpass

browser = webdriver.Chrome(service=Service(ChromeDriverManager().install()))

s = Service("/usr/local/bin/chromedriver")
browser = webdriver.Chrome(service=s)

course = "98897122-22s_swen"
url = f"https://classroom.github.com/classrooms/{course}"

github_account = "dominik.boehler@gmx.net"
github_passwd = getpass("input your github password:")

browser.get(url)

browser.find_element(By.ID("login_field")).clear()
browser.find_element(By.ID("login_field")).send_keys(github_account)
browser.find_element(By.ID("password")).clear()
browser.find_element(By.ID("password")).send_keys(github_passwd)
browser.find_element(By.Name("commit")).click()
browser.find_element(By.Link_text("Repositories")).click()


time.sleep(10)
browser.quit()
