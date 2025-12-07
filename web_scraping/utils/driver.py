from data.data import Data
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.chrome.service import Service
from webdriver_manager.chrome import ChromeDriverManager
import time
import os, glob, shutil
import platform
from pathlib import Path


class AdminTHAWebDriver:
    def __init__(self):
        """ 
        Custom driver instance of Chrome is configured 
        """
        # files are saved in your os Downloads file
        home_dir = Path.home()
        self.downloads_dir = os.path.join(home_dir, "Downloads")
        if os.path.exists(self.downloads_dir):
    
            # options are set to use different browser modes like heedless, cognito etc.
            options = webdriver.ChromeOptions()
            
            #options.add_argument("--headless")  # Run in headless mode (no UI), no pop up


            self.driver = webdriver.Chrome(service=Service(ChromeDriverManager().install()), options=options)
            self.driver.implicitly_wait(5)
        else: 
            print("Download path doesnt exist.")



    def open_website(self):
        """
        Opens a website, here: "THA: Prüfungsportal"
        """
        self.driver.get(Data.URL)
        
    def fill_credentials(self):
        """
        Fills the login fields with username and password
        """  
        # pass your username
        username_input = self.driver.find_element(By.ID, "asdf")
        username_input.clear()
        username_input.send_keys(Data.USERNAME)
        
        # pass your password
        pw_input = self.driver.find_element(By.ID, "fdsa")
        pw_input.clear()
        pw_input.send_keys(Data.PW)
        
        login_button = self.driver.find_element(By.ID, "loginForm:login")
        login_button.click()
        
        self.logged = True
        
    def go_to_download_page(self):
        """
        When user is logged in the admin page is opened
        """
        if self.logged:
            prüfungsverwaltung = self.driver.find_element(By.CLASS_NAME, "auflistung") 
            prüfungsverwaltung.click()
            
            notenspiegel = self.driver.find_element(By.XPATH, "//*[@id=\"wrapper\"]/div[6]/div[2]/div/form/div/ul/li[3]/a") 
            notenspiegel.click()
            
            
    def download_pdf(self):
        """
        Clicks on the pdf-link in notenspiegel and 
        after download completes the file is copied 
        from the local "Downloads" folder into this projects "files" folder.
        """
        old_length = len(os.listdir(self.downloads_dir))
        pdf_link = self.driver.find_element(By.XPATH, "//*[@id=\"wrapper\"]/div[6]/div[2]/a[2]/img")
        pdf_link.click()
        files_dir = os.path.join(os.getcwd(), "files" )
        print("😎 PDF-Datei wird heruntergeladen. Das kann einige Sekunden dauern... ")
        while True:
            time.sleep(10) # so that download is not continuosly interrupted 
            files = [f for f in glob.glob(f"{self.downloads_dir}/*") if os.path.isfile(f)] # stores only the files
            files.sort(key=os.path.getmtime, reverse=True) # sorts files by recently modified 
            if files: # in case download order is empty 
                if len(os.listdir(self.downloads_dir)) > old_length: # means that download started
                    if not files[0].endswith(".crdownload"): #  indicates a finished download
                        print(f"Datei wurde erfolgreich heruntergeladen: {files[0]}.")
                        shutil.copy2(files[0], files_dir)
                        return files[0]
                        
    def close_session(self):
        self.driver.quit()