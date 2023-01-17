import { Worker, NearAccount } from 'near-workspaces';
import anyTest, { TestFn } from 'ava';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;

  // Create test account alice
  const alice = await root.createSubAccount('alice');
  const xcc = await root.createSubAccount('xcc');
  const helloNear = await root.createSubAccount('hello-near');

  await helloNear.deploy("./src/hello-near/hello-near.wasm");
  await xcc.deploy(process.argv[2]);

  xcc.call(xcc, "init", {hello_account: helloNear.accountId})

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, helloNear, xcc, alice };
});

test.afterEach.always(async (t) => {
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('returns the default greeting', async (t) => {
  const { xcc, alice } = t.context.accounts;

  const greeting: string = await alice.call(xcc, 'query_greeting', {}, { gas: "200000000000000" });
  t.is(greeting, 'Hello');
});

test('changes the greeting', async (t) => {
  const { xcc, alice } = t.context.accounts;
  const res = await alice.call(xcc, 'change_greeting', { new_greeting: 'Howdy' }, { gas: "200000000000000" });
  t.is(res, true);
  
  const greeting: string = await alice.call(xcc, 'query_greeting', {}, { gas: "200000000000000" });
  t.is(greeting, 'Howdy');
});