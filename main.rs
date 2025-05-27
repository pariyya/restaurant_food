use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use colored::*;
use rand::Rng;

fn save_to_json<T: Serialize>(data: &[T], file_name: &str) -> io::Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    let mut file = File::create(file_name)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_from_json<T: for<'a> Deserialize<'a>>(file_name: &str) -> io::Result<Vec<T>> {
    if !Path::new(file_name).exists() {
        return Ok(Vec::new());
    }
    let mut file = File::open(file_name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let data: Vec<T> = serde_json::from_str(&content)?;
    Ok(data)
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Restaurant {
    restaurant_name: String,
    restaurant_category: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct FoodMenu {
    restaurant_name: String,
    food_name: String,
    price: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Owner {
    owner_name: String,
    owner_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct User {
    name: String,
    password: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct ShoppingCart {
    food_name: String,
    price: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Order {
    restaurant_name: String,
    customer_name: String,
    food_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Admin {
    admin_name: String,
    admin_id: u32,
}

fn gen_security_code() -> u32 {
    let mut rng = rand::thread_rng();
    rng.random_range(1000..=9999)
}

fn register_user() -> io::Result<()> {
    let mut users = load_from_json("users.json")?;
    
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    println!("Enter password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password: u32 = match password.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid password! Must be a number.");
            return Ok(());
        }
    };

    let security_code = gen_security_code();
    println!("Security code: {}", security_code);

    println!("Enter the security code:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let code: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid security code!");
            return Ok(());
        }
    };

    if code != security_code {
        println!("Security code does not match!");
        return Ok(());
    }

    if users.iter().any(|u: &User| u.name == name) {
        println!("Username already exists!");
    } else {
        users.push(User { name, password });
        save_to_json(&users, "users.json")?;
        println!("Registration successful!");
    }

    Ok(())
}

fn view_restaurants() -> io::Result<()> {
    let restaurants: Vec<Restaurant> = load_from_json("restaurants.json")?;
    
    if restaurants.is_empty() {
        println!("No restaurants available!");
        return Ok(());
    }

    println!("\n{}", "--- Available Restaurants ---".green());
    println!("{:-<60}", "");
    println!("{:<30} {:<25}", 
            "Restaurant Name".cyan(), 
            "Category".cyan());
    println!("{:-<30} {:-<25}", "", "");

    for restaurant in restaurants {
        println!("{:<30} {:<25}",
                restaurant.restaurant_name.purple(),
                restaurant.restaurant_category);
    }

    Ok(())
}

fn view_menu() -> io::Result<()> {
    let restaurants: Vec<Restaurant> = load_from_json("restaurants.json")?;
    let foods: Vec<FoodMenu> = load_from_json("foods.json")?;
    
    println!("Enter restaurant name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let rest_name = name.trim();

    if !restaurants.iter().any(|r| r.restaurant_name == rest_name) {
        println!("Restaurant not found!");
        return Ok(());
    }

    let restaurant_foods: Vec<&FoodMenu> = foods.iter()
        .filter(|f| f.restaurant_name == rest_name)
        .collect();

    if restaurant_foods.is_empty() {
        println!("No menu items found for this restaurant!");
    } else {
        println!("\nMenu for '{}':", rest_name);
        println!("{:-<40}", "");
        println!("{:<25} {:<10}", "Food Name", "Price");
        println!("{:-<25} {:-<10}", "", "");

        for food in restaurant_foods {
            println!("{:<25} ${:<9.2}", food.food_name, food.price);
        }
    }
    
    Ok(())
}

fn add_to_cart() -> io::Result<()> {
    let mut cart = load_from_json("cart.json")?;
    let foods: Vec<FoodMenu> = load_from_json("foods.json")?;
    
    println!("Enter food name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let food_name = name.trim();

    if let Some(food) = foods.iter().find(|f| f.food_name == food_name) {
        cart.push(ShoppingCart {
            food_name: food.food_name.clone(),
            price: food.price,
        });
        save_to_json(&cart, "cart.json")?;
        println!("Added to cart successfully!");
    } else {
        println!("Food not found!");
    }

    Ok(())
}

fn delete_from_cart() -> io::Result<()> {
    let mut cart: Vec<ShoppingCart> = load_from_json("cart.json")?;
    
    if cart.is_empty() {
        println!("Your cart is empty!");
        return Ok(());
    }

    println!("\nItems in your cart:");
    for (index, item) in cart.iter().enumerate() {
        println!("{}. {} - ${:.2}", index + 1, item.food_name, item.price);
    }

    println!("\nEnter food name to remove:");
    let mut food_name = String::new();
    io::stdin().read_line(&mut food_name)?;
    let food_name = food_name.trim();

    if let Some(index) = cart.iter().position(|item| item.food_name == food_name) {
        cart.remove(index);
        save_to_json(&cart, "cart.json")?;
        println!("Item removed from cart!");
    } else {
        println!("Item not found in cart!");
    }

    Ok(())
}


fn register_owner() -> io::Result<()> {
    let mut owners = load_from_json("owners.json")?;
    
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    println!("Enter owner ID:");
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input)?;
    let owner_id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ID! Must be a number.");
            return Ok(());
        }
    };

    let security_code = gen_security_code();
    println!("Security code: {}", security_code);

    println!("Enter the security code:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let code: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid security code!");
            return Ok(());
        }
    };

    if code != security_code {
        println!("Security code does not match!");
        return Ok(());
    }

    if owners.iter().any(|o: &Owner| o.owner_name == name || o.owner_id == owner_id) {
        println!("Owner already exists with this name or ID!");
    } else {
        owners.push(Owner {
            owner_name: name,
            owner_id,
        });
        save_to_json(&owners, "owners.json")?;
        println!("Owner registered successfully!");
    }
    Ok(())
}

fn register_restaurant() -> io::Result<()> {
    let mut restaurants = load_from_json("restaurants.json")?;
    
    println!("Enter restaurant name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let rest_name = name.trim().to_string();

    if restaurants.iter().any(|r: &Restaurant| r.restaurant_name == rest_name) {
        println!("Restaurant with this name already exists!");
        return Ok(());
    }

    println!("Enter restaurant category:");
    let mut category = String::new();
    io::stdin().read_line(&mut category)?;
    let rest_category = category.trim().to_string();

    restaurants.push(Restaurant {
        restaurant_name: rest_name,
        restaurant_category: rest_category,
    });
    
    save_to_json(&restaurants, "restaurants.json")?;
    println!("Restaurant added successfully!");
    Ok(())
}

fn make_menu() -> io::Result<()> {
    let mut foods = load_from_json("foods.json")?;
    let restaurants = load_from_json("restaurants.json")?;
    
    println!("Enter restaurant name:");
    let mut rest_name = String::new();
    io::stdin().read_line(&mut rest_name)?;
    let rest_name = rest_name.trim();

    if !restaurants.iter().any(|r: &Restaurant| r.restaurant_name == rest_name) {
        println!("Restaurant not found!");
        return Ok(());
    }

    println!("Enter food name:");
    let mut food_name = String::new();
    io::stdin().read_line(&mut food_name)?;
    let food_name = food_name.trim().to_string();

    if foods.iter().any(|f: &FoodMenu| f.food_name == food_name && f.restaurant_name == rest_name) {
        println!("Food already exists in this restaurant's menu!");
        return Ok(());
    }

    println!("Enter price:");
    let mut price_input = String::new();
    io::stdin().read_line(&mut price_input)?;
    let price: f64 = match price_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid price! Must be a number.");
            return Ok(());
        }
    };

    foods.push(FoodMenu {
        restaurant_name: rest_name.to_string(),
        food_name,
        price,
    });
    
    save_to_json(&foods, "foods.json")?;
    println!("Food added to menu successfully!");
    Ok(())}
    fn view_orders() -> io::Result<()> {
    let carts: Vec<ShoppingCart> = load_from_json("cart.json").unwrap_or_default();
    let restaurants: Vec<Restaurant> = load_from_json("restaurants.json")?;

    println!("Enter restaurant name to view its orders:");
    let mut rest_name = String::new();
    io::stdin().read_line(&mut rest_name)?;
    let rest_name = rest_name.trim();

    
    if !restaurants.iter().any(|r| r.restaurant_name == rest_name) {
        println!("{}", "Restaurant not found!".red());
        return Ok(());
    }

    
    if carts.is_empty() {
        println!("{}", "No orders found for this restaurant!".yellow());
    } else {
        println!("\n{} '{}':", "Orders for".green().bold(), rest_name.purple().bold());
        println!("{:-<60}", "");
        println!("{:<20} {:<20} {:<15}", 
                "Customer".cyan().bold(), 
                "Food".cyan().bold(), 
                "Price".cyan().bold());
        println!("{:-<20} {:-<20} {:-<15}", "", "", "");

        for item in carts {
            println!("{:<20} ${:<14.2}",
                    item.food_name.purple(),
                    item.price);
        }
    }

    Ok(())
}
fn register_admin() -> io::Result<()> {
    let mut admins = load_from_json("admins.json")?;
    
    println!("Enter admin username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let admin_name = username.trim().to_string();

    println!("Enter admin ID:");
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input)?;
    let admin_id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ID! Must be a number.");
            return Ok(());
        }
    };

    let security_code = gen_security_code();
    println!("Security code: {}", security_code);

    println!("Enter the security code:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let code: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid security code!");
            return Ok(());
        }
    };

    if code != security_code {
        println!("Security code does not match!");
        return Ok(());
    }

    if admins.iter().any(|a: &Admin| a.admin_name == admin_name || a.admin_id == admin_id) {
        println!("Admin with this username or ID already exists!");
    } else {
        admins.push(Admin {
            admin_name,
            admin_id,
        });
        save_to_json(&admins, "admins.json")?;
        println!("Admin registered successfully!");
    }
    Ok(())
}

fn view_users() -> io::Result<()> {
    let users: Vec<User> = load_from_json("users.json")?;
    let admins: Vec<Admin> = load_from_json("admins.json")?;
    
    println!("Enter admin username:");
    let mut admin_name = String::new();
    io::stdin().read_line(&mut admin_name)?;
    let admin_name = admin_name.trim();

    println!("Enter admin ID:");
    let mut admin_id = String::new();
    io::stdin().read_line(&mut admin_id)?;
    let admin_id: u32 = match admin_id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid admin ID!");
            return Ok(());
        }
    };

    if !admins.iter().any(|a| a.admin_name == admin_name && a.admin_id == admin_id) {
        println!("Admin authentication failed!");
        return Ok(());
    }

    println!("\nRegistered Users:");
    println!("{:-<30}", "");
    println!("{:<20} {:<10}", "Username", "Password");
    println!("{:-<20} {:-<10}", "", "");

    for user in users {
        println!("{:<20} {:<10}", user.name, user.password);
    }

    Ok(())
}

fn delete_user() -> io::Result<()> {
    let mut users: Vec<User> = load_from_json("users.json")?;
    let admins: Vec<Admin> = load_from_json("admins.json")?;
    
    println!("Enter admin username:");
    let mut admin_name = String::new();
    io::stdin().read_line(&mut admin_name)?;
    let admin_name = admin_name.trim();

    println!("Enter admin ID:");
    let mut admin_id = String::new();
    io::stdin().read_line(&mut admin_id)?;
    let admin_id: u32 = match admin_id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid admin ID!");
            return Ok(());
        }
    };

    if !admins.iter().any(|a: &Admin| a.admin_name == admin_name && a.admin_id == admin_id) {
        println!("Admin authentication failed!");
        return Ok(());
    }

    println!("\nCurrent Users:");
    for (i, user) in users.iter().enumerate() {
        println!("{}. {}", i + 1, user.name);
    }

    println!("\nEnter username to delete:");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim();

    if let Some(index) = users.iter().position(|u| u.name == username) {
        users.remove(index);
        save_to_json(&users, "users.json")?;
        println!("User deleted successfully!");
    } else {
        println!("User not found!");
    }

    Ok(())
}
fn main() -> io::Result<()> {
    loop {
        println!("\n--- Main Menu ---");
        println!("1. User");
        println!("2. Owner");
        println!("3. Admin");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match choice {
            1 => {
 
                loop {
                    println!("\n--- User Menu ---");
                    println!("1. Register");
                    println!("2. View restaurants");
                    println!("3. View menu");
                    println!("4. Add to cart");
                    println!("5. Delete from cart");
                    println!("6. Back");

                    let mut user_choice = String::new();
                    io::stdin().read_line(&mut user_choice)?;
                    let user_choice: u32 = match user_choice.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please enter a valid number!");
                            continue;
                        }
                    };

                    match user_choice {
                        1 => register_user()?,
                        2 => view_restaurants()?,
                        3 => view_menu()?,
                        4 => add_to_cart()?,
                        5 => delete_from_cart()?,
                        6 => break,
                        _ => println!("Invalid choice!"),
                    }
                }
            },
            2 => {
 
                loop {
                    println!("\n--- Owner Menu ---");
                    println!("1. Register owner");
                    println!("2. Register restaurant");
                    println!("3. Make menu");
                    println!("4. View orders");
                    println!("5. Back");

                    let mut owner_choice = String::new();
                    io::stdin().read_line(&mut owner_choice)?;
                    let owner_choice: u32 = match owner_choice.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please enter a valid number!");
                            continue;
                        }
                    };

                    match owner_choice {
                        1 => register_owner()?,
                        2 => register_restaurant()?,
                        3 => make_menu()?,
                        4 => view_orders()?,
                        5 => break,
                        _ => println!("Invalid choice!"),
                    }
                }
            },
            3 => {
 
                loop {
                    println!("\n--- Admin Menu ---");
                    println!("1. Register admin");
                    println!("2. View users");
                    println!("3. Delete user");
                    println!("4. Back");

                    let mut admin_choice = String::new();
                    io::stdin().read_line(&mut admin_choice)?;
                    let admin_choice: u32 = match admin_choice.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please enter a valid number!");
                            continue;
                        }
                    };

                    match admin_choice {
                        1 => register_admin()?,
                        2 => view_users()?,
                        3 => delete_user()?,
                        4 => break,
                        _ => println!("Invalid choice!"),
                    }
                }
            },
            4 => {
                println!("Exiting program...");
                break;
            },
            _ => println!("Invalid choice!"),
        }
    }

    Ok(())
}
