// @ts-check

import assert from 'assert';
import uws from 'uws';

const app = uws.App({});

app.ws('/*', {
  open: (ws) => {
    console.log('uws: open');
    ws.send('Hello');
  },
  message: (ws, message, is_binary) => {
    console.log('uws: message');
    if (is_binary === false) {
      console.log(Buffer.from(message).toString(), is_binary);
    } else {
      console.log(Buffer.from(message));
    }
  },
  close: (ws) => {
    console.log('uws: close');
  },
});

app.get('/*', (res) => {
  res.writeStatus('200');
  res.writeHeader('Content-Type', 'text/plain');
  res.write('Hello world!');
  res.end();
});

app.listen(8080, 1, (token) => {
  assert(token !== false, 'uws: app.listen failed.');
  console.log('uws: listening at port 8080');
});