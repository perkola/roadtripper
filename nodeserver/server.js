var dotenv = require('dotenv').config();
var url = require('url');
var express = require("express");
var https = require('https');
var app = express();

app.post("/api/citydistance", function(req, res) {
    var body = [];
    req.on('data', function(chunk) {
        body.push(chunk);
    });

    req.on('end', function() {
        var query = JSON.parse(body.toString());

        console.log(query);

        var options = {
            host: 'maps.googleapis.com',
            path: '/maps/api/distancematrix/json?'
                + 'origins=' + query['from']
                + '&destinations=' + query['to']
                + '&key=' + process.env['GOOGLE_KEY'],
            method: 'GET'
        };

        console.log(options);

        return https.get(options, function(response) {
            var body = [];
            return response.on('data', function(data) {
                body.push(data);
            }).on('end', function() {
                body = Buffer.concat(body).toString();
                return res.send(JSON.parse(body));
            });
        });
    });
    /* some server side logic */
    /*var url_parts = url.parse(req.url, true);
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
            return res.send(data.toString('utf8'));
        });
    });*/
});

app.get("/api/roadtrip", function(req, res) {
    var url_parts = url.parse(req.url, true);
    var query = url_parts.query;
    var token = query['token'];

    var fs = require('fs');
    if (!(fs.existsSync('../roadtrips/' + token + '.rdt'))) {
        return res.status(500).send('No such roadtrip');
    }

    var content;
    fs.readFile('../roadtrips/' + token + '.rdt', function read(err, data) {
        if (err) {
            throw err;
        }
        content = data;
        return res.send(JSON.parse(content));
    });

})

app.post("/api/autocomplete", function(req, res) {

    var body = [];
    req.on('data', function(chunk) {
        body.push(chunk);
    });

    req.on('end', function() {
        var cityCompletion = encodeURI(body.toString());

        var options = {
            host: 'maps.googleapis.com',
            path: '/maps/api/place/autocomplete/json?'
                + 'input=' + cityCompletion
                + '&types=(cities)'
                + '&radius=20000000'
                + '&key=' + process.env['GOOGLE_KEY'],
            method: 'GET'
        }

        console.log(options);

        return https.get(options, function(response) {
            var body = [];
            return response.on('data', function(data) {
                body.push(data);
            }).on('end', function() {
                body = Buffer.concat(body).toString();
                return res.send(JSON.parse(body));
            });
        });
    });
});

app.post("/api/save", function(req, res) {
    var token = tokenGen(10);

    var fs = require('fs');
    if (!(fs.existsSync('../roadtrips'))) {
        fs.mkdir('../roadtrips');
    }

    req.on('data', function(chunk) {
        fs.writeFile('../roadtrips/' + token + '.rdt', chunk.toString(), function(err) {
            if (err) {
                return res.send("ERROR");
            }
            console.log("Wrote to file");
        });
    });

    res.send(token);
});

function tokenGen(n) {
    var text = "";

    var charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    for (var i = 0; i < n; i++) {
        text += charset.charAt(Math.floor(Math.random() * charset.length));
    }

    return text;
}

var port = 8080; //The port on which your server listens
app.use(express.static('../front/'));
console.log("Listening on port " + port + "...");
app.listen(port);
