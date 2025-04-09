// Import necessary modules
use cursive::views::{Dialog, EditView, ListView}; 
use cursive::{Cursive, CursiveExt}; 
use cursive::traits::{Nameable, Resizable};

use std::sync::{Arc, Mutex}; 
use std::fs::{File, OpenOptions}; 
use std::io::{self, Read}; 
use serde::{Serialize, Deserialize};
use cursive::theme::{BaseColor, BorderStyle, Color, ColorStyle, Palette, PaletteColor, Theme};

// necessary modules to play clicking sound
use rodio::{Decoder, OutputStream, Sink};
use std::io::BufReader;
use std::thread;
use std::time::Duration;

fn play_click_sound() {
    //println!("Trying to play click sound...");

    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        //println!("Got stream handle.");
        if let Ok(sink) = Sink::try_new(&stream_handle) {
            //println!("Sink created.");
            if let Ok(file) = File::open("C:/Users/deeks/OneDrive/Documents/Java_Projects/click1.wav") {
                //println!("Sound file opened.");
                let source = Decoder::new(BufReader::new(file)).unwrap();
                sink.append(source);
                thread::sleep(Duration::from_millis(300));
                //sink.detach();
                //sink.sleep_until_end(); // TEMPORARY: lets you hear the sound before moving on

            } else {
                println!("Failed to open file.");
            }
        } else {
            println!("Failed to create sink.");
        }
    } else {
        println!("Failed to get default output stream.");
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)] 
struct Product { 
    product_type: String, 
    quantity: usize,
    price_per_unit: f64, 
    sales_tax: f64, 
    total_price: f64, 
}

// Define a constant for the file path where inventory data will be stored.
const FILE_PATH: &str = "inventory.json"; 

// Function to save products to a JSON file.
fn save_products_to_file(products: &Vec<Product>) -> io::Result<()> { 
    let file = OpenOptions::new() // Create a new OpenOptions instance to configure file opening.
        .write(true) 
        .create(true) 
        .truncate(true) 
        .open(FILE_PATH)?; 
    
    serde_json::to_writer(file, products)?; // Serialize the products vector to JSON and write it to the file.
    Ok(()) 
}

// Function to load products from the JSON file.
fn load_products_from_file() -> Vec<Product> { 
    if let Ok(mut file) = File::open(FILE_PATH) { 
        let mut data = String::new(); 
        if file.read_to_string(&mut data).is_ok() { 
            if let Ok(products) = serde_json::from_str::<Vec<Product>>(&data) {
                return products; 
            }
        }
    }
    Vec::new() 
}

fn custom_theme() -> Theme {
    let mut palette = Palette::default();

    // Customize specific colors
    palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Primary] = Color::Dark(BaseColor::White);
    palette[PaletteColor::TitlePrimary] = Color::Light(BaseColor::Cyan);
    palette[PaletteColor::HighlightText] = Color::Light(BaseColor::Red);
    palette[PaletteColor::Highlight] = Color::Dark(BaseColor::White);
    palette[PaletteColor::Secondary] = Color::Light(BaseColor::Green);
    palette[PaletteColor::Tertiary] = Color::Dark(BaseColor::Yellow);

    Theme {
        palette,
        shadow: true,
        borders: BorderStyle::Simple,
    }
}

fn main() { 
    // Create a new instance of Cursive for the UI.
    let mut siv = Cursive::default(); 
    siv.set_theme(custom_theme());

    // Load products from file and wrap them in Arc and Mutex for safe shared access.
    let products = Arc::new(Mutex::new(load_products_from_file())); 

    // Add a dialog layer to the UI for managing the inventory.
    siv.add_layer(
        Dialog::new()
            .title("Inventory Manager") 
            .content(ListView::new() 
                .child("Product Type:", EditView::new().with_name("product_type")) 
                .child("Quantity:", EditView::new().with_name("quantity")) 
                .child("Price per Unit:", EditView::new().with_name("price_per_unit")) 
            )
            .button("Save", {
                let products_clone = Arc::clone(&products);
                move |s| { 
                    play_click_sound();
                    let product_type = s 
                        .call_on_name("product_type", |view: &mut EditView| {
                            view.get_content()
                        })
                        .unwrap() 
                        .to_string(); 

                    let quantity = s 
                        .call_on_name("quantity", |view: &mut EditView| {
                            view.get_content()
                        })
                        .unwrap() 
                        .parse::<usize>() //parse as usize
                        .unwrap_or(0); // If parsing fails, default to 0.

                    let price_per_unit = s 
                        .call_on_name("price_per_unit", |view: &mut EditView| {
                            view.get_content()
                        })
                        .unwrap() 
                        .parse::<f64>() 
                        .unwrap_or(0.0); 

                    // Validation: Check if the fields are empty or invalid.
                    if product_type.is_empty() { 
                        //s.add_layer(Dialog::info("Error: Please enter a product type.")); 
                        s.add_layer(
                            Dialog::text("Error: Please enter a product type.")
                                .button("Ok", |s| {
                                     play_click_sound();
                                    s.pop_layer();
                                })
                        );
                        return; // Exit the closure.
                    }

                    if quantity == 0 { 
                        s.add_layer(
                            Dialog::text("Error: Please enter a valid quantity.")
                                .button("Ok", |s| {
                                     play_click_sound();
                                    s.pop_layer();
                                })
                        );
                        return; 
                    }

                    if price_per_unit == 0.0 { 
                       // s.add_layer(Dialog::info("Error: Please enter a valid price.")); 
                       s.add_layer(
                        Dialog::text("Error: Please enter a valid price.")
                            .button("Ok", |s| {
                                 play_click_sound();
                                s.pop_layer();
                            })
                        );
                        return; 
                    }

                    let sales_tax = 0.20 * price_per_unit; 
                    let total_price = (price_per_unit + sales_tax) * quantity as f64; 

                    let product = Product { 
                        product_type,
                        quantity,
                        price_per_unit,
                        sales_tax,
                        total_price,
                    };

                    let mut product_store = products_clone.lock().unwrap(); // Lock the Mutex to safely access the products.
                    product_store.push(product.clone()); // Add the new product to the product store.

                    // Save to file
                    if let Err(err) = save_products_to_file(&product_store) { 
                        //s.add_layer(Dialog::info(format!()); 
                        s.add_layer(
                            Dialog::info(format!("Error saving product: {}", err))
                                .button("Ok", |s| {
                                     play_click_sound();
                                    s.pop_layer();
                                })
                        );
                        return; 
                    } else {
                        //s.add_layer(Dialog::info("Product saved successfully!"));
                        s.add_layer(
                            Dialog::text("Product saved successfully")
                                .button("Ok", |s| {
                                     play_click_sound();
                                    s.pop_layer();
                                })
                        );
                        
                    }
                }
            })
            .button("Show All", { 
                let products = Arc::clone(&products); // Clone the Arc for thread-safe access.
                move |s| { 
                    play_click_sound();
                    let product_store = products.lock().unwrap(); // Lock the Mutex to access the products.
                    let mut output = String::new(); 

                    for (index, product) in product_store.iter().enumerate() { 
                        output.push_str(&format!( 
                            "{}. Item: {}, Qty: {}, Price: ${}, Sales Tax: ${}, T.Price: ${}\n",
                            index + 1, 
                            product.product_type, 
                            product.quantity, 
                            product.price_per_unit, 
                            product.sales_tax, 
                            product.total_price,
                        ));
                    }

                    if output.is_empty() { 
                        output = "No products in the inventory.".to_string(); 
                    }

                    //s.add_layer(Dialog::info(output)); 
                    s.add_layer(
                        Dialog::info(output)
                            .button("Ok", |s| {
                                 play_click_sound();
                                s.pop_layer();
                            })
                    );
                }
            })
            .button("Delete by ID", { 
                let products_clone = Arc::clone(&products); // Clone the Arc for thread-safe access.
                move |s| { 
                    play_click_sound();
                    // Get ID from user
                    let id_input = EditView::new().with_name("delete_id").min_width(10); 
                    s.add_layer(Dialog::new() 
                        .title("Delete Product") 
                        .content(ListView::new() 
                            .child("Enter product ID to delete:", id_input) 
                        )
                        .button("Confirm", { 
                            let products_clone = Arc::clone(&products_clone); 
                            move |s| { 
                                play_click_sound();
                                let id_str = s 
                                    .call_on_name("delete_id", |view: &mut EditView| {
                                        view.get_content()
                                    })
                                    .unwrap() 
                                    .to_string(); 

                                // Parse ID
                                if let Ok(id) = id_str.parse::<usize>() { 
                                    let mut product_store = products_clone.lock().unwrap(); 

                                    // Check if ID is valid
                                    if id > 0 && id <= product_store.len() { 
                                        product_store.remove(id - 1); 
                                        if let Err(err) = save_products_to_file(&product_store) { 
                                            //s.add_layer(Dialog::); 
                                            s.add_layer(
                                                Dialog::info(format!("Error deleting product: {}", err))
                                                    .button("Ok", |s| {
                                                         play_click_sound();
                                                        s.pop_layer();
                                                    })
                                            );
                                        } else {
                                            //s.add_layer(Dialog::info("Product deleted successfully!")); 
                                            s.add_layer(
                                                Dialog::info("Product deleted successfully!")
                                                    .button("Ok", |s| {
                                                         play_click_sound();
                                                        s.pop_layer();
                                                    })
                                            );
                                        }
                                    } else {
                                        //s.add_layer(Dialog::info("Error: Invalid product ID.")); 
                                        s.add_layer(
                                            Dialog::info("Error: Invalid product ID.")
                                                .button("Ok", |s| {
                                                     play_click_sound();
                                                    s.pop_layer();
                                                })
                                        );
                                    }
                                } else {
                                    //s.add_layer(Dialog::info("Error: Please enter a valid number.")); 
                                    s.add_layer(
                                        Dialog::info("Error: Please enter a valid number.")
                                            .button("Ok", |s| {
                                                 play_click_sound();
                                                s.pop_layer();
                                            })
                                    );
                                }
                            }
                        })
                        .button("Cancel",{
                            |s| {
                                play_click_sound();
                                s.pop_layer(); 
                            }
                        })
                    );
                }
            })
            .button("Quit",{
                |s|{
                    play_click_sound();
                    s.quit();
                } 
            })
    );

    siv.run(); 
}