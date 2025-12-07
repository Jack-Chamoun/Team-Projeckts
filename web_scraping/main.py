import pdfplumber
from src.extract_pdf import PdfExtractor

class Calculator:
    def __init__(self, pdf_path):
        self.pdf_path = pdf_path
        self.all_rows = []
        self.data = []

    def extract_pdf_info(self):
        """Extracts all text rows from the PDF"""
        with pdfplumber.open(self.pdf_path) as pdf:
            for page in pdf.pages:
                text = page.extract_text()
                if text:
                    self.all_rows.extend(text.split('\n'))
        
    def extract_subjects_info(self):
        """Parses relevant subject information from extracted rows"""

        for row in self.all_rows:
            values = row.split()

            # ensure we are only processing subject-related lines (those starting with a course number)
            if len(values) >= 6 and values[0].isdigit():
                first_column = values[0]  # first column, the subject code
                third_last = values[-3].replace(',', '.')  # third last column, the grade
                last_column = values[-2].replace(',', '.') # last column, the ects
                
                # discard any subject that does not have any grade (practical, seminar etc.)
                try:
                    last_column = float(last_column)
                    self.data.append((first_column, third_last, last_column))
                except ValueError:
                    continue  

    def calculate_ects_sum(self):
        """Calculates the total sum of ECTS credits"""
        return sum(row[2] for row in self.data)

    def calculate_average_grade(self):
        """Calculates the weighted average grade"""
        sum_of_grades = 0
        sum_of_ects = 0
        for row in self.data: 
            try:
                grade = float(row[1])
                subject_code = int(row[0])//10
                if (int(subject_code) // 1000 == 397) & (subject_code % 100 <= 10):
                    sum_of_grades += (grade * row[2] * 0.5)
                    sum_of_ects += (row[2] * 0.5)
                else:
                    sum_of_grades += (grade * row[2])
                    sum_of_ects += (row[2])
            except ValueError:
                continue
        return round((sum_of_grades/sum_of_ects), 3)

    def run(self):
        """Runs the full process"""
        self.extract_pdf_info()
        self.extract_subjects_info()
        ects_sum = self.calculate_ects_sum()
        average_grade = self.calculate_average_grade()
        print(f"Die Summe aller ECTS beträgt: {ects_sum}.")
        print(f"Dein Notendurchschnitt ist: {average_grade}.")

if __name__ == "__main__":
    pdf_extractor = PdfExtractor()
    pdf_path = pdf_extractor.get_pdf()

    if pdf_path:
        calculator = Calculator(pdf_path)
        calculator.run()
    else:
        print("❌ Failed to download the PDF.")
