from pathlib import Path
import pickle, gzip, math, os, time, shutil, matplotlib as mpl, matplotlib.pyplot as plt
import json  # Use 'import csv' for CSV format
from urllib.request import urlretrieve

MNIST_URL='https://github.com/mnielsen/neural-networks-and-deep-learning/blob/master/data/mnist.pkl.gz?raw=true'
path_data = Path('data')
path_data.mkdir(exist_ok=True)
path_gz = path_data/'mnist.pkl.gz'
MNIST_PATH = Path('data/mnist')

if not path_gz.exists(): urlretrieve(MNIST_URL, path_gz)

# Load the pickle file
with gzip.open(path_gz, 'rb') as f: 
    ((x_train, y_train), (x_valid, y_valid), _) = pickle.load(f, encoding='latin-1')

# Convert and save to JSON
with open(MNIST_PATH/'x_valid.json', 'w') as file:
    json.dump(x_valid.tolist(), file)

# For CSV, you would use the csv module and adjust the code accordingly.
