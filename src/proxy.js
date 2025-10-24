const Srf = require('drachtio-srf');
const srf = new Srf();

srf.connect({
	host: '172.29.0.10',
	port: 9022,
	secret: 'cymru'
});

//srf.use((req, res, next) => console.log(`incoming ${req.method} from ${req.source_address}}`));

srf.on('connect', (err, hostport) => {
	console.log(`connected to a drachtio server listening on: ${hostport}`);
});

srf.on('error', (err) => {
	console.log(`Error: ${err}`);
});

srf.cancel((req, res) => {
	console.log(`received CANCEL`);
	console.log(`++++++++++++++++++`);
	res.send(200, {
		headers: {
			'X-Custom-Header': 'yo dude?'
		}
	});
});

srf.invite(async (req, res) => {
	console.log(`received INVITE from ${req.source_address}`);
	console.log('proxying request');
	const originAddress = req.source_address;
	const originPort = req.source_port;

	const proxyOptions = {
		headers: {
			'X-Stop': 100,
		},
		followRedirects: false,
		forking: 'sequential',
		remainInDialog: true,
		provisionalTimeout: '1s',
		finalTimeout: '3s'
	};

	console.log(JSON.stringify(req));

	srf.request(`sip:${originAddress}:${originPort}`, {
		method: 'OPTIONS',
		headers: {
			'User-Agent': 'drachtio-ducks'
		}
	}, (err, req) => {
		req.on('response', (res) => {
			console.log(`received ${res.statusCode} response`);
		});
	});



	let result = await srf.proxyRequest(
		req,
		['sip:+happy_feet@172.29.0.11;transport=UDP'],
		proxyOptions);

	console.log(result.finalStatus);
});

srf.register((req, res) => {
	console.log(`received REGISTER`);
	res.send(200, {
		headers: {
			'X-Custom-Header': 'because why not?'
		}
	});
});
