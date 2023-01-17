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
  const escrow = await root.createSubAccount('escrow-account');
  const ownerA = await root.createSubAccount('owner-a');
  const ownerB = await root.createSubAccount('owner-b');

  await contract.deploy(
    process.argv[2],
  );

  t.context.worker = worker;
  t.context.accounts = { root, contract, escrow, ownerA, ownerB };
});

test.afterEach.always(async (t) => {
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('everything in contract', async (t) => {
  const { root, contract, escrow, ownerA, ownerB } = t.context.accounts;
  const asset = await root.call(contract, 'init', {asset_price: "10", escrow_contract_id: escrow.accountId, total_supply: "100", owner_id: ownerA.accountId});
  const total_supply: string = await contract.view('get_total_supply', {});
  t.is(total_supply, '100');
  const assets_of_a: string = await contract.view('get_account_assets', {account_id: ownerA.accountId});
  t.is(assets_of_a, '100');

  await escrow.call(contract, 'purcharse_asset', { seller_account_id: ownerA, buyer_account_id: ownerB, attached_near: "100"});
  const assets_of_a_new: string = await contract.view('get_account_assets', {account_id: ownerA.accountId});
  const assets_of_b_new: string = await contract.view('get_account_assets', {account_id: ownerB.accountId});
  t.is(assets_of_a_new, '90');
  t.is(assets_of_b_new, '10');

  await escrow.call(contract, 'transfer_asset', { quantity: "10", from_account_id: ownerA, to_account_id: ownerB});
  const assets_of_a_ne: string = await contract.view('get_account_assets', {account_id: ownerA.accountId});
  const assets_of_b_ne: string = await contract.view('get_account_assets', {account_id: ownerB.accountId});
  t.is(assets_of_a_ne, '80');
  t.is(assets_of_b_ne, '20');
});
