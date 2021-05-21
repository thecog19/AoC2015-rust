struct Rectangle {
    xmin: i64,
    ymin: i64,
    xmax: i64,
    ymax: i64,
    light: u32
}

struct Line {
    x_min: i64,
    y_min: i64,
    y_max: i64,
    x_max: i64,
}

fn main(){
    let line_1 = Line { x_min: 0, y_min: 0, x_max: 0, y_max: 10 };
    let line_2 = Line { x_min: 0, y_min: 3, x_max: 0, y_max: 10 } ;

    print!("{}", ovelappling_line_segements(line_1, line_2));
}

fn ovelappling_line_segements(line_1: Line, line_2: Line ) -> bool{

    let mut line_1_horizontal = true;
    let mut line_2_horizontal = true;

    if line_1.x_min - line_1.x_max == 0{
        line_1_horizontal = false;
    }
 
    if line_2.x_min - line_2.x_max == 0{
        line_2_horizontal = false;
    }
    
    if (line_1_horizontal == line_2_horizontal){
        // they are colinear, is there an overlap?

        // DOES THE smallest point on line 2 lie on line 1? 
        if (line_1.y_min <= line_2.y_min && line_1.y_max >= line_2.y_min){
            return true
        }
        // Does the largest point on line 2 lie on line 1? 
        else if (line_1.y_min <= line_2.y_max && line_1.y_max >= line_2.y_max){
            return true
        }
        //does the smallest point on line 1 lie on line 2? 
        else if (line_1.y_min >= line_2.y_min && line_1.y_min <= line_2.y_max){
            return true
        }

        //does the smallest point on line 1 lie on line 2? 
        else if (line_1.y_max >= line_2.y_min && line_1.y_max <= line_2.y_max){
            return true
        }
    }

    if (line_1_horizontal != line_2_horizontal){
        // they are colinear, is there an overlap?

        // DOES THE smallest point on line 2 lie on line 1? 
        if (line_1.x_min <= line_2.x_min && line_1.x_max >= line_2.x_min){
            return true
        }
        // Does the largest point on line 2 lie on line 1? 
        else if (line_1.x_min <= line_2.x_max && line_1.x_max >= line_2.x_max){
            return true
        }
        //does the smallest point on line 1 lie on line 2? 
        else if (line_1.x_min >= line_2.x_min && line_1.x_min <= line_2.x_max){
            return true
        }
        
        //does the smallest point on line 1 lie on line 2? 
        else if (line_1.x_max >= line_2.x_min && line_1.x_max <= line_2.x_max){
            return true
        }
    }

    return false

}
