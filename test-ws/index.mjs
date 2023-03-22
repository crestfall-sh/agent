// @ts-check

import assert from 'assert';
import uws from 'uws';

const app = uws.App({});

app.ws('/*', {
  open: (ws) => {
    console.log('uws: open');
    ws.send('HELLO');
  },
  message: (ws, message, is_binary) => {
    console.log('uws: message');
    if (is_binary === false) {
      const message_text = Buffer.from(message).toString();
      console.log(message_text, is_binary);
      switch (message_text) {
        case 'HI': {
          ws.send(JSON.stringify({ type: 'TEST' }));
          break;
        }
        default: {
          break;
        }
      }
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