use std::fs::File;
use std::io::{BufRead, BufReader};
use linfa_clustering::KMeans;
use linfa::traits::Fit;
use linfa::prelude::Predict;
use std::collections::HashMap;
use statistical::{mean, standard_deviation};
use ndarray::Array2;
use std::error::Error;
use linfa::dataset::DatasetBase;

fn main() -> Result<(), Box<dyn Error>> {
    let mut complete_flights = read_csv("src/International_Report_Departures.csv").unwrap();

    let rows_to_keep: Vec<usize> = (2..1003).collect();
    let updated_list:Vec<(i32,i32)> = complete_flights.into_iter().enumerate().filter(|(i,_)| rows_to_keep.contains(i)).map(|(_,row)| row).collect();
    complete_flights = updated_list;

    data_standardization(& mut complete_flights);

    let kmax = 100;
    let mut n_clusters: HashMap<usize, usize> = HashMap::new();

    for k in 2..=kmax {
        let n_rows = complete_flights.len();
        let n_columns = 2;
        let flat: Vec<f32> = complete_flights.iter().flat_map(|&(us_id, fg_id)| vec![us_id as f32, fg_id as f32]).collect();
        let flights = Array2::from_shape_vec((n_rows, n_columns), flat)?;
        
        let clusters = kmeans_analysis(&flights, k)?;
    

    for(i,cluster) in clusters.iter().enumerate() {
        let (_,_fg_id) = complete_flights[i];
        let how_many = n_clusters.entry(*cluster).or_insert(0);
        *how_many += 1;
    }
 }

    let mut max = 0;
    let mut max_fg_id = Vec::new();

    for (fg_id, how_many) in n_clusters {
        if how_many > max {
            max = how_many;
            max_fg_id.clear();
            max_fg_id.push(fg_id);
        } else if max == how_many {
            max_fg_id.push(fg_id);
        }
    }

     println!("International Airport ID: {:?}", (max));

     Ok(())
}

fn read_csv(path: &str) -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
    let mut result: Vec<(i32, i32)> = Vec::new();
    let csv_file = File::open(path)?;
    let mut buf_reader = BufReader::new(csv_file);

    let mut first_row = String::new();
    buf_reader.read_line(&mut first_row)?;

    let rows = buf_reader.lines().skip(1);
    for row in rows {
        let row_str = row?;
        let r: Vec<&str> = row_str.trim().split(",").collect();
        let us_id = r[3].parse::<i32>()?;
        let fg_id = r[6].parse::<i32>()?;
        result.push((us_id, fg_id));
    }
    Ok(result)
}

fn data_standardization(flights: &mut Vec<(i32,i32)>) {
    let us_id_values: Vec<f32> = flights.iter().map(|&(us_id, _)| us_id as f32).collect();
    let fg_id_values: Vec<f32> = flights.iter().map(|&(_,fg_id)| fg_id as f32).collect();
    let mean_us_id:f32 = mean(&us_id_values);
    let mean_fg_id:f32= mean(&fg_id_values);
    let stddev_us_id = standard_deviation(&us_id_values, Some(mean_us_id));
    let stddev_fg_id= standard_deviation(&fg_id_values, Some(mean_fg_id));

    for (us_id, fg_id) in flights {
        *us_id = (((*us_id as f32) - mean_us_id) / stddev_us_id) as i32;
        *fg_id = (((*fg_id as f32) - mean_fg_id) / stddev_fg_id) as i32;
    }
}

fn kmeans_analysis(flights: &Array2<f32>, k:usize) -> Result<Vec<usize>, Box<dyn Error>> {
    let data = DatasetBase::new(flights.view(), ());
    let model = KMeans::params(k).max_n_iterations(250).fit(&data)?;
   
    Ok(model.predict(&data).to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_kmeans_analysis() {
        let test_data = Array2::from_shape_vec((2,3), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let k = 2;
        let clusters = kmeans_analysis(&test_data, k).unwrap();
        
        assert_eq!(clusters.len(), 2);
    }

    #[test]
    fn test2_data_standardization() {
        let mut test_data = vec![(10,20), (30,40)];
        data_standardization(&mut test_data);

        assert_eq!(test_data, vec![(0,0), (0,0)]);
    }
} 