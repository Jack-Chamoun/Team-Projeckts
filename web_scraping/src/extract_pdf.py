from utils.driver import AdminTHAWebDriver

class PdfExtractor:
    def get_pdf(self):
        webdriver = AdminTHAWebDriver()
        webdriver.open_website()
        webdriver.fill_credentials()
        webdriver.go_to_download_page()
        pdf_path = webdriver.download_pdf()
        webdriver.close_session()
        return pdf_path
        
        