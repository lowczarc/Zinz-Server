# Zinz server

### Getting started
You need a redis server accessible on `redis://127.0.0.1/`

### Usage

You can record a user location making a POST request at `/location/USERNAME?latitude=LAT&longitude=LNG`

Making a GET request to `/location/USERNAME` will give you the last location recorded and when it was recorded
