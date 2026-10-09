use std::io;
use std::f64::consts::PI;

fn main() {
    println!("Hello, user! Welcome To The Shape Calculator. Feel free to calculate any shape you want.");
    println!("What is your name user?: ");
    let mut name= String::new();
    io::stdin().read_line(&mut name).expect("Not a valid name."); 
     println!("Ok  {} here are some of our shapes: ", name);
     println!("\nTrapezium,\nRhombus,\nParallelogram,\nCube\nCylinder");
     //When the user is done with code,user should type done.
     println!("You can write them in snakecase or capitalize the first word to get started.");
     println!("Have fun using the program to calculate the area and volume of shapes.\nDo not forget to be creative!");

     //Call add fuction with arguements
 loop {
     let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Failed to read line");
     let shape = input.trim().to_uppercase();

     if shape == "done" {
        break;
     }
    
 if shape == "Trapezium" {
    trapezium()
 }
 else if shape == "Rhombus"  {
    rhombus()
 }
 else if shape == "Parallelogram"  {
    parallelogram()
 }
 else if shape == "Cube" {
    cube()
 }
 else if shape == "Cylinder" {
    cylinder ()
  } 
 }
}
fn trapezium() {
    let mut input1 = String::new();
    println!("Enter the float value for base1 :");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base_1: f64 = input1.trim().parse().expect("Invalid Input");
      
    let mut input2 = String::new();
    println!("Enter the float value for base2 :");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base_2: f64 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter the float value for height: "); 
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let height: f64 = input3.trim().parse().expect("Invalid input");
    let area = 0.5 * height * (base_1 + base_2);

    println!("The area of the Trapezium = {}", area);
}


fn rhombus() {
    let mut input1 = String::new();
    println!("Enter the float value for the first diagonal: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let diagonal_1:f64 = input1.trim().parse().expect("Not a valid input");
    
    let mut input2 = String::new();
    println!("Enter the float value for the second diagonal: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let diagonal_2:f64 = input2.trim().parse().expect("Invalid input");

    let area = 0.5 * diagonal_1 * diagonal_2;
    println!("The area of The Rhombus is: {} ", area);
}

fn parallelogram()  {

    let mut input1 = String::new();
    println!("Enter the float value for the base: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base: f64 = input1.trim().parse().expect("Invalid input");
    
    let mut input2 = String::new();
    println!("Enter the float value for the altitude :");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let altitude:f64 = input2.trim().parse().expect("Invalid input");
    

    let area = base * altitude;
    println!("The area of the Parallelogram is: {}", area);
}

fn cube() {
    
    let mut input1 = String::new();
    println!("Enter the float value of the side of the cube.");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let side:f64 = input1.trim().parse().expect("Invalid input");
    
    let surface_area = 6.0 * side * side;
    println!("The surface area of the cube  is: {}", surface_area);

    println!("Do you also wish to calculate the volume of the cube as well? (Y/N)");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read input");
    let response = response.trim();

    if response == "Y"{
        println!("Alrighty then let's get started.");
        let mut input2 = String::new();
        println!("Enter the float value of the side of the cube: ");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let side: f64 = input1.trim().parse().expect("Invalid input");
        //Two different sides in case the use wants to find 
        //the volume of a cube with a different side. 
        let volume = side * side * side;
        println!("The volume of the Cube is : {}",volume);
    }

    else if response == "N"{
            println!("Thank you for trying");
     }

    else {
            println!("You might want to review the character you typed.");
    }
 
}

fn cylinder() {
    // 1. Get Radius
    let mut input_radius = String::new();
    println!("Enter the value of the radius of the cylinder: ");
    io::stdin()d.read_line(&mut input_radius).expect("Failed to read input");
    let radius: f64 = input_radius.trim().parse().expect("Invalid float for radius");

    // 2. Get Height
    let mut input_height = String::new();
    println!("Enter the float value of the height of the cylinder: ");
    io::stdin().read_line(&mut input_height).expect("Failed to read input");
    let height: f64 = input_height.trim().parse().expect("Invalid float for height");

    // 3. Use standard PI constant
    let pi = PI;

    // 4. Calculate Volume
    let volume = pi * radius * radius * height;
    println!("The volume of the Cylinder is: {:.7}", volume);

    // 5. Ask for Surface Area
    println!("\nDo you want to calculate the Surface areas of the Cylinder too? (Y/N)");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read character");
    let response = response.trim().to_uppercase();

    if response == "Y" {
        println!("Alright then.");

        // Reuse existing radius and height
        let total_surface_area = 2.0 * pi * radius * (radius + height);
        let curved_surface_area = 2.0 * pi * radius * height;

        println!("The Total Surface area of the Cylinder is: {:.4}", total_surface_area);
        println!("The Curved Surface Area of the Cylinder is: {:.4}", curved_surface_area);
    } else if response == "N" {
        println!("Thank you for trying.");
    } else {
        println!("You might want to review what you typed.");
    }
}

/*fn cylinder () {
    use std::f64::consts::PI;

    let mut input1 = String::new();
    println!("Enter the value of the radius of the cylinder: ");
    io::stdin().read_line(&mut input1).expect("Invalid input maybe it's meant to be a float.");
    let radius:f64 = input1.trim().parse().expect("Invalid float");
    

    let input2 = String::new();
    println!("Enter the float value of the height of the cylinder: ");
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let height:f64 = input2.trim().parse().expect("Invalid float");

    let input4 = String::new();
    println!("Enter the float value of pi of the cylinder (most preferably 3.141592654): ");
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let pi:f64 = input4.trim().parse().expect("Invalid float");
    //3.141592654

    let pi = PI;

    let volume = pi * radius * radius * height;
    println!("The volume of the Cylinder is: {:.7}", volume);

    println!("\nDo you want to calculate the Surface areas of the Cylinder too? (Y/N)");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read character");
    let response = response.trim().to_uppercase();
    
    if response == "Y"{
        println!("Alright then.");

         let mut input1 = String::new();
         println!("Enter the float value of the radius of the cylinder: ");
         io::stdin().read_line(&mut input1).expect("Invalid input maybe it's meant to be a float or a string..");
         let radius: f64 = input1.trim().parse().expect("Invalid float");
        
         let mut input2 = String::new();
         println!("Enter the float value of the height of the cylinder: ");
         io::stdin().read_line(&mut input1).expect("Invalid input");
         let height: f64 = input2.trim().parse().expect("Invalid float");
         

         let Total_Surface_Area =  2.0 * pi * radius * (radius + height);
         println!("The Total Surface area of the Cylinder is: {:.4}", Total_Surface_Area);

         let Total_Curved_Surface_Area = pi * radius * (2.0 * height + radius);
         println!("The Curved Surface Area of the Cylinder is: {:.4}", Total_Curved_Surface_Area );  
    }
    else if response == "N"{
        println!("Thank you for trying.");
    }

    else{
        println!("You might want to review what you typed.");
    }*/

