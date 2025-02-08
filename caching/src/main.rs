use std::collections::HashMap;
use redis::{Commands, RedisResult, Client};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub id: i32,
    pub name: String,
    pub cars: Vec<String>,
    pub children: HashMap<String,i32>,
}

impl Data {
    fn default() -> Self {
        let mut children = HashMap::new();
        children.insert("James".to_string(), 23);
        children.insert("Karen".to_lowercase(), 34);
        Data {
           id: 1,
            name: "Samson".to_string(),
            cars: vec!["Subaru Imprezza".to_string(), "Mazda CX5".to_string()],
            children, 
        }
    }

    fn add_car(&mut self, car: String) {
        self.cars.push(car);
    }

    fn print(&self) {
        println!("\nID       : {}\nName     : {:}\nCars     : {}\nChildren : {:?}\n-------------", self.id, self.name, self.cars.join(","), self.children);
    }

    fn add_child(&mut self, name: String, age: i32) {
        self.children.insert(name, age);
    }

    fn save(&self, client: &Client) -> Result<(), String> {
        let mut con = client.get_connection().map_err(|e| e.to_string())?;
        let serialized = serde_json::to_string(self).map_err(|e| e.to_string())?;
        con.set("caching_data", serialized).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn read(client: &Client) -> Result<Option<Self>, String> {
        let mut con = client.get_connection().map_err(|e| e.to_string())?;
        let data: Option<String> = con.get("caching_data").map_err(|e| e.to_string())?;
        match data {
            Some(json) => {
                let data = serde_json::from_str(&json).map_err(|e| e.to_string())?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

}

 fn save(data: &Data, client: &Client) -> Result<(), String> {
        let mut con = client.get_connection().map_err(|e| e.to_string())?;
        let serialized = serde_json::to_string(data).map_err(|e| e.to_string())?;
        con.set("caching_data", serialized).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn read(client: &Client) -> Result<Option<Data>, String> {
        let mut con = client.get_connection().map_err(|e| e.to_string())?;
        let data: Option<String> = con.get("caching_data").map_err(|e| e.to_string())?;
        match data {
            Some(json) => {
                let data = serde_json::from_str(&json).map_err(|e| e.to_string())?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }





fn main() {
    let client = Client::open("redis://127.0.0.1/").expect("Failed to connect to redis");

    let mut data = Data::default();
    data.print();
   
    data.add_car("Toyota Corolla".to_string());
    data.add_child("Terry".to_string(), 25);
    data.print();

   if let Err(e) = save(&data, &client) {
        println!("failed to save because {}", e);
    }

   match read(&client) {
       Ok(Some(d)) => println!("Retrieved data: {:?}", d),
       Ok(None) => println!("Data not found in redis"),
       Err(e) => println!("Error retrieving player: {e}"),
   }

}
