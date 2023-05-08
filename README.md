# ds_210_air_traffic_project
The project applies k-means clustering on a csv file named "International_Report_Departures.csv" (linked below). This csv file contains the international flights departured from the US between the years 1990-2020. The goal was to find the international airport that receives most of the international flights departured from the US. By determining this, improvements can be made in that airport to accomodate large number of passengers from the US. A directed graph was formed from this dataset and each airport was a vertices, while each flight was an edge. Thus, each row of the dataset represents one flight (1 edge).

For this goal, the program starts by reading the csv file and selecting only the columns containing the unique 5 digit id for US and international aiports. Since the dataset contained around 1000000 edges and around 40000 vertices, the dataset used for this analysis was reduced to 1002 edges for running time purposes. Then, to get a evenly distributed analysis, dataset was standardized by substracting the mean from each id number and diving it by their standard deviation. Also, some more minor data cleaning was applied. Using the linfa_clustering::Kmeans crate, kmeans analysis was conducted on the now standardized dataset. This clustering analysis was then backtracked to find which international airport ID had the most clustering.

To run the project, the csv file should be downloaded from the link below. If necessary, under the main function, the file name should be changed so that the program can read the csv. As a side note, since only a random selection of 1002 vertices are used in this analysis, everytime the code is executed, different 1002 vertices can be selected which might end up producing different results. It had to be a random selection to avoid bias in the analysis (for example if it was the first 1000 entries in the dataset, these first 1000 entries might have arranged in an alphabetical order).

RESULTS \
When I ran the code 5 times back to back, I got the result of 'International Airport ID: 12938'. From the original dataset, it can be seen that ID: 12938 belongs to the Leipzig/Halle Airport in Germany. It can be concluded that with the random 1002 vertices I have used in the kmeans analysis, the international airport that received most of the international flights from the US was Leipzig/Halle Airport.

Dataset obtained from: https://www.kaggle.com/datasets/parulpandey/us-international-air-traffic-data?resource=download \
Name of Dataset: 'International_Report_Departures.csv'

Resources used: 
- https://docs.rs/linfa-clustering/latest/linfa_clustering/struct.KMeans.html
- https://docs.rs/linfa-clustering/latest/linfa_clustering/struct.KMeansParams.html
- https://jeffbelgum.github.io/statistical/statistical/
- https://docs.rs/linfa-clustering/0.5.1/linfa_clustering/struct.KMeans.html
- https://rust-ml.github.io/book/3_kmeans.html
- DS210 lecture notes
