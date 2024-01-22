const Srf = require('drachtio-srf');
const srf = new Srf();

srf.connect({
	host: '172.29.0.11',
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

srf.invite((req, res) => {
	console.log(`received INVITE`);

	console.log(res.get('to'));
	console.log(req.msg.raw);

	if (req.msg.headers['X-Stop']) {
		console.log(`received X-Stop header, sending busy response`);
		res.send(486, 'So sorry, busy right now', {
			headers: {
				'X-Custom-Header': 'because why not?'
			}
		});
		return;
	}

	res.send(603, 'Not Accepted Here');
});

srf.register((req, res) => {
	console.log(`received REGISTER`);
});
