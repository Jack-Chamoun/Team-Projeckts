import dotenv,os

dotenv.load_dotenv()
class Data:
    URL="https://www.verwaltung.fh-augsburg.de/qisserver/rds?state=user&type=0"
    USERNAME=os.getenv("WEB_USERNAME")
    PW= os.getenv("WEB_PW")
    