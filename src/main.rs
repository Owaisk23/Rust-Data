struct report_card{
    Gpa: f64,
    Grade: String,
    Total_Marks: f64
}

impl report_card{
    fn progress(&self,next_year:&report_card)->bool{
            self.Gpa > next_year.Gpa
    }
}


fn main() {
    let year1 = report_card{
        Gpa: 3.7,
        Grade:String::from("A+"),
        Total_Marks: 445.0
    };

      let year2 = report_card{
        Gpa: 3.3,
        Grade:String::from("B"),
        Total_Marks: 378.0
    };

      let year3 = report_card{
        Gpa: 2.9,
        Grade:String::from("C"),
        Total_Marks: 345.0
    };

      let year4 = report_card{
        Gpa: 3.6,
        Grade:String::from("A"),
        Total_Marks: 420.0
    };
   
    let all_Gpa = [year1.Gpa, year2.Gpa, year3.Gpa, year4.Gpa];
    
    let gain_marks = [year1.Total_Marks, year2.Total_Marks, year3.Total_Marks, year4.Total_Marks];
    
    let Best_year = highest_gpa(&all_Gpa);
    println!("The highest Gpa is: {}", Best_year);
    let g = Overall_marks(&gain_marks);
    println!("The overall marks is: {}", g);

    println!("The year1 Gpa is greater then year 2: {}", year1.progress(&year2));
    println!("The year3 Gpa is greater then year 4: {}", year3.progress(&year4));

}

fn Overall_marks(max:&[f64;4])->f64{
    let mut result:f64 = 0.0;
    for x in 0..3{
        result = result + max[x];
    }
    result
}

fn highest_gpa(max_gpa:&[f64;4])->f64{
    let mut initial = max_gpa[0];

    for i in 0..3{
        if max_gpa[i] > initial{
            initial = max_gpa[i]
        }
    }
    initial
}