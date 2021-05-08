export function initClient() {

  const initClient = hasuraWs.buildClient((address) => {
    const ws = new WebSocket(address, 'graphql-ws');
    ws.on = (type, listener) =>
      ws.addEventListener(type, (event) => listener(event.data));
    return ws;
  });

  // TODO: Proper auth process
  const client = initClient({
    address: 'wss://api.vx-template-app.network/v1/graphql', // (either ws:// or wss://)
    debug: false, // log additional information for sent and recieved messages
    // Crendentials :
    adminSecret: 'vx-template-app-access-key', // your hasura secret
  });

  return client;
}

export function subscribe(client, query, callback) {
  const subscription = client.subscribeFromString(
    // TODO: Avoid stringify. Can we get JSON from sub?
    (data) => callback(JSON.stringify(data)),
    query
  );
  return subscription;
}

export function unsubscribe(subscription) {
  const { execution, unsubscribe } = subscription;
  unsubscribe();
}