var dotenv = require('dotenv').config();
var url = require('url');
var express = require("express");
var https = require('https');
var app = express();

app.get("/api/citydistance", function(req, res) {
    /* some server side logic */
    var url_parts = url.parse(req.url, true);
    var query = url_parts.query;
    var froms = encodeURI(query['from']);
    var to = encodeURI(query['to']);

    var options = {
        host: 'maps.googleapis.com',
        path: '/maps/api/distancematrix/json?'
            + 'origins=' + froms
            + '&destinations=' + to
            + '&key=' + process.env['GOOGLE_KEY'],
        method: 'GET'
    };

    options['path'] = encodeURI(options['path']);

    console.log(options);

    https.get(options, function(response) {
        response.on('data', function(data) {
            console.log(data.toString('utf8'));
            res.send(data.toString('utf8'));
        });
    });
});

var port = 8080; //The port on which your server listens
app.use(express.static('../front/'));
console.log("Listening on port " + port + "...");
app.listen(port);
