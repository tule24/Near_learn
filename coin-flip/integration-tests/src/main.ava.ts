import { Worker, NearAccount } from 'near-workspaces';
import anyTest, { TestFn } from 'ava';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const contract = await root.createSubAccount('test-account');
  const alice = await root.createSubAccount('alice-account');
  
  await contract.deploy(
    process.argv[2],
  );

  t.context.worker = worker;
  t.context.accounts = { root, contract, alice };
});

test.afterEach.always(async (t) => {
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('by default the user has no points', async (t) => {
  const { contract, alice } = t.context.accounts;
  const point: number = await contract.view('get_points_by_account', {account: alice.accountId});
  t.is(point, 0);
});

test('the points are correctly computed', async (t) => {
  const { contract, alice } = t.context.accounts;
  
  let counter: {[key: string]: number} = { 'Heads': 0, 'Tails': 0 };
  let expect_point = 0;

  for (let i = 0; i < 10; i++) {
    const res = await alice.call(contract, 'flip_coin', { 'player_guess': 'Heads'})
    counter[res as 'string'] += 1;
    expect_point += res == 'Heads' ? 1 : -1;
    expect_point = Math.max(expect_point, 0);
  }

  // A binomial(10, 1/2) has a P(x > 2) ~ 98%
  t.true(counter['Heads'] >= 2);

  const points: number = await contract.view('get_points_by_account', { 'account': alice.accountId});
  t.is(points, expect_point)
});