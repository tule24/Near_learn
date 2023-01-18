import { Worker, NearAccount } from 'near-workspaces';
import anyTest, { TestFn } from 'ava';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

type PremiumMessage = {
  premium: boolean;
  sender: string;
  text: string;
}

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;

  // Create test account
  const alice = await root.createSubAccount('alice');
  const xcc = await root.createSubAccount('xcc');
  const helloNear = await root.createSubAccount('hellonear');
  const guestBook = await root.createSubAccount('guestbook');
  const counter = await root.createSubAccount('counter');

  // Deploy external contract
  await helloNear.deploy("./src/external-contracts/hello-near.wasm");
  await counter.deploy("./src/external-contracts/counter.wasm");
  await guestBook.deploy("./src/external-contracts/guest-book.wasm");

  // Deploy xcc contract
  await xcc.deploy(process.argv[2]);

  // Initialize xcc contract
  await xcc.call(xcc, "init", {
    hello_account: helloNear,
    counter_account: counter,
    guestbook_account: guestBook
  });

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, alice, xcc, helloNear, guestBook, counter };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('multiple_contract test', async (t) => {
  const { alice, xcc, helloNear, counter, guestBook } = t.context.accounts;
  
  await alice.call(counter, "decrement", {});
  await alice.call(helloNear, "set_greeting", { greeting: "Howdy"});
  await alice.call(guestBook, "add_message", { text: "my message" }, { gas: "40000000000000" });

  const res: [string, number, [PremiumMessage]] = await alice.call(xcc, "multiple_contracts", {}, { gas: "300000000000000" }); // 3e14
  const expected = {
    premium: false,
    sender: "alice.test.near",
    text: "my message"
  };
  
  t.is(res[0], "Howdy");
  t.is(res[1], -1);
  t.deepEqual(res[2], [expected]);
  t.pass();
});

test('batch_action test', async (t) => {
  const { alice, xcc } = t.context.accounts;
  const res: string = await alice.call(xcc, "batch_actions", {}, {gas: "300000000000000"}); // 3e14
  t.deepEqual(res, "bye");
});

test('similar_contracts test', async (t) => {
  const { alice, xcc } = t.context.accounts;
  
  const res: [[string]] = await alice.call(xcc, "similar_contracts", {}, {gas: "300000000000000"}); // 3e14
  const expected = ["hi", "howdy", "bye"];

  t.deepEqual(res, expected);
});
