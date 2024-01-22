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

srf.invite(async (req, res) => {
	console.log(`received INVITE`);

	console.log('proxying request');
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

	let result = await srf.proxyRequest(
		req,
		['sip:+happy_feet@172.29.0.11;transport=UDP'],
		proxyOptions);

	console.log(result.finalStatus);
});

srf.register((req, res) => {
	console.log(`received REGISTER`);
});
