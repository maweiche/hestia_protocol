import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HestiaProtocol } from "../target/types/hestia_protocol";
import { PublicKey, Keypair, Transaction , MemcmpFilter, GetProgramAccountsConfig, Connection, SYSVAR_INSTRUCTIONS_PUBKEY, sendAndConfirmTransaction} from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createAssociatedTokenAccountIdempotentInstruction, createInitializeMint2Instruction, createMintToInstruction, getAssociatedTokenAddressSync, getMinimumBalanceForRentExemptMint, MINT_SIZE, TOKEN_PROGRAM_ID, } from "@solana/spl-token";

describe("hestia_protocol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HestiaProtocol as Program<HestiaProtocol>;
  const wallet = anchor.Wallet.local();
  const provider = anchor.getProvider();
  const connection = new Connection('http://localhost:8899', 'confirmed');

  const mint = Keypair.generate();
  const customer = Keypair.generate();

  //  it("Setup Mint and Accounts", async () => {
  //   let lamports = await getMinimumBalanceForRentExemptMint(connection);
  //   let tx = new Transaction();
  //   const ownerAta = await getAssociatedTokenAddressSync(mint.publicKey, wallet.publicKey, true);
  //   tx.instructions = [
  //     anchor.web3.SystemProgram.createAccount({fromPubkey: wallet.publicKey, newAccountPubkey: mint.publicKey, lamports, space: MINT_SIZE, programId: TOKEN_PROGRAM_ID}),
  //     createInitializeMint2Instruction(mint.publicKey, 6, wallet.publicKey, null),
  //     createAssociatedTokenAccountIdempotentInstruction(wallet.publicKey, ownerAta, wallet.publicKey, mint.publicKey),
  //     createMintToInstruction(mint.publicKey, ownerAta, wallet.publicKey, 1e9),
  //   ];

  //   await provider.sendAndConfirm(tx, [wallet.payer, mint]);

  //   let tx2 = new Transaction();
  //   const customerAta = await getAssociatedTokenAddressSync(mint.publicKey, customer.publicKey, true);
  //   tx2.instructions = [
  //     createAssociatedTokenAccountIdempotentInstruction(wallet.publicKey, customerAta, customer.publicKey, mint.publicKey),
  //     createMintToInstruction(mint.publicKey, customerAta, wallet.publicKey, 1e9),
  //   ];

  //   await provider.sendAndConfirm(tx2, [wallet.payer, mint]);
  // });

  it("Protocol is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .protocolInit()
      .signers([wallet.payer])
      .rpc();
    // console.log("Your transaction signature", tx);
  });

  it("Protocol is toggled to lock!", async () => {
    // Add your test here.
    const tx = await program.methods.protocolToggle().rpc();
    // console.log("Your transaction signature", tx);
  });

  it("Protocol is toggled to unlock!", async () => {
    // Add your test here.
    const tx = await program.methods.protocolToggle().rpc();
    // console.log("Your transaction signature", tx);
  });

  let newAdmin = Keypair.generate();
  let newAdminUsername = 'steve';
  let newAdminProfile = PublicKey.findProgramAddressSync([Buffer.from('admin'), newAdmin.publicKey.toBuffer()], program.programId)[0];
  
  it("Add new Admin of Protocol!", async () => {
    // Add your test here.
    const tx = await program.methods
      .protocolAddAdmin(
        newAdminUsername
      )
      .accountsPartial({
        owner: wallet.publicKey,
        newAdmin: newAdmin.publicKey,
        adminProfile: newAdminProfile,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    // // console.log("Your transaction signature", tx);
  });

  it("Add new Admin of Protocol!", async () => {
    // Add your test here.
    const tx = await program.methods
      .protocolRemoveAdmin()
      .accountsPartial({
        admin: newAdmin.publicKey,
        adminProfile: newAdminProfile,
        primaryAdmin: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    // console.log("Your transaction signature", tx);
  });

  const newRestaurantOwner = Keypair.generate();
  const newRestaurant = Keypair.generate();
  const newRestaurantOwnerProfile = PublicKey.findProgramAddressSync([Buffer.from('admin'), newRestaurantOwner.publicKey.toBuffer()], program.programId)[0];
  const [newRestaurantPda, newRestaurantPdaBump] = PublicKey.findProgramAddressSync([Buffer.from('restaurant'), newRestaurantOwner.publicKey.toBuffer()], program.programId);
  const id = Math.floor(Math.random() * 1000000);
  const restaurantType = 0;
  const restaurantName = 'Kentucky Fried Chicken';
  const restaurantSymbol = 'KFC';
  // const restaurantCurrency = new PublicKey('4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU');
  const url = 'https://www.kfc.com';

  const createRestaurantArgs = {
    id: new anchor.BN(id),
    restaurantType: restaurantType,
    name: restaurantName,
    symbol: restaurantSymbol,
    currency: mint.publicKey,
    url: url,
    bump: newRestaurantPdaBump,
  }

  it("Add new Restaurant to the Protocol!", async () => {
    // airdrop 1 sol to newRestaurantOwner
    const connection = new Connection('http://localhost:8899', 'confirmed');
    const airdrop = await connection.requestAirdrop(newRestaurantOwner.publicKey, 1e10);
    console.log('airdrop!!', airdrop);
    // add 2 second timeout
    await new Promise(resolve => setTimeout(resolve, 2000));
    // Add your test here.
    const tx = await program.methods
      .restaurantInitialize(createRestaurantArgs)
      .accountsPartial({
        restaurantAdmin: newRestaurantOwner.publicKey,
        adminProfile: newRestaurantOwnerProfile,
        restaurant: newRestaurantPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([newRestaurantOwner])
      .rpc();

    console.log("Your transaction signature", tx);
  });

  const newEmployee = Keypair.generate();
  const [employeeProfile, employeeBump] = PublicKey.findProgramAddressSync([Buffer.from('employee'), newRestaurantPda.toBuffer(), newEmployee.publicKey.toBuffer()], program.programId);
  const employeeType = 0;
  const username = 'bill'

  const newEmployeeArgs = {
    wallet: newEmployee.publicKey,
    restaurant: newRestaurant.publicKey,
    employeeType: employeeType,
    username: username,
    bump: employeeBump,
  };

  it("Add new Employee to restaurant!", async () => {
    // Add your test here.
    const tx = await program.methods
      .restaurantAddEmployee(newEmployeeArgs)
      .accountsPartial({
        employee: employeeProfile,
        restaurantAdmin: newRestaurantOwner.publicKey,
        adminProfile: newRestaurantOwnerProfile,
        restaurant: newRestaurantPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([newRestaurantOwner])
      .rpc();
    // console.log("Your transaction signature", tx);
  });

  const today = new Date();
  const sku = Math.floor(Math.random() * 1000000)
  // seeds = [b"inventory", restaurant.key().as_ref(), args.sku.to_le_bytes().as_ref()],
  const [itemPda, itemBump] = PublicKey.findProgramAddressSync([Buffer.from('inventory'), newRestaurantPda.toBuffer(), Buffer.from(sku.toString())], program.programId);
  const addInventoryArgs = {
    sku: new anchor.BN(sku),
    category: 2,
    name: 'Chicken Wings',
    price: new anchor.BN(10),
    stock: new anchor.BN(100),
    lastOrder: new anchor.BN(today.getTime()),
    initialized: false,
    bump: itemBump,
  };

  it("Add new Inventory Item to restaurant!", async () => {
    // Add your test here.
    const tx = await program.methods
      .restaurantAddInventoryItem(addInventoryArgs)
      .accountsPartial({
        item: itemPda,
        restaurantAdmin: newRestaurantOwner.publicKey,
        adminProfile: newRestaurantOwnerProfile,
        restaurant: newRestaurantPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([newRestaurantOwner])
      .rpc();
    // console.log("Your transaction signature", tx);
  });

  const updateInventoryArgs = {
    sku: new anchor.BN(sku),
    category: 2,
    name: 'Chicken Wings',
    description: '6 pieces of chicken wings',
    price: new anchor.BN(1),
    stock: new anchor.BN(10),
    lastOrder: new anchor.BN(today.getTime()),
    initialized: true,
    bump: itemBump,
  };

  it("Update Inventory Item in restaurant!", async () => {
    // Add your test here.
    const tx = await program.methods
      .restaurantAddInventoryItem(updateInventoryArgs)
      .accountsPartial({
        item: itemPda,
        restaurantAdmin: newRestaurantOwner.publicKey,
        adminProfile: newRestaurantOwnerProfile,
        restaurant: newRestaurantPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([newRestaurantOwner])
      .rpc();
    // console.log("Your transaction signature", tx);
  });

  const menuItemSku = (Math.floor(Math.random() * 1000000));
  const [menuItemPda, menuItemBump] = PublicKey.findProgramAddressSync([Buffer.from('item'), newRestaurantPda.toBuffer(), Buffer.from(menuItemSku.toString())], program.programId);
  const [menuPda, menuBump] = PublicKey.findProgramAddressSync([Buffer.from('menu'), newRestaurantPda.toBuffer()], program.programId);
  const menuItemArgs = {
    sku: new anchor.BN(menuItemSku),
    category: 0,
    name: 'Chicken Wings',
    price: new anchor.BN(10),
    description: '6 pieces of chicken wings',
    ingredients: [sku.toString()],
    active: false
  };

  it("Add item to restaurant Menu!", async () => {
    // Add your test here.
    const tx = await program.methods
      .restaurantAddMenuItem(menuItemArgs)
      .accountsPartial({
        item: menuItemPda,
        menu: menuPda,
        restaurantAdmin: newRestaurantOwner.publicKey,
        adminProfile: newRestaurantOwnerProfile,
        restaurant: newRestaurantPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([newRestaurantOwner])
      .rpc();
    // console.log("Your transaction signature", tx);
  });

  const toggleMenuItemArgs = {
    sku: new anchor.BN(menuItemSku),
    active: true
  }

  it("Toggle menu item!", async () => {
    // Add your test here.
    const tx = await program.methods
      .restaurantToggleMenuItem(toggleMenuItemArgs)
      .accountsPartial({
        item: itemPda,
        menu: menuPda,
        restaurantAdmin: newRestaurantOwner.publicKey,
        adminProfile: newRestaurantOwnerProfile,
        restaurant: newRestaurantPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([newRestaurantOwner])
      .rpc();
    // console.log("Your transaction signature", tx);
  });

  
  const [customerPda, customerBump] = PublicKey.findProgramAddressSync([Buffer.from('customer'), newRestaurantPda.toBuffer() , customer.publicKey.toBuffer()], program.programId);
  const orderId = (Math.floor(Math.random() * 1000000));
  const [orderPda, orderBump] =  PublicKey.findProgramAddressSync([Buffer.from('order'), newRestaurantPda.toBuffer(), Buffer.from(orderId.toString()), customer.publicKey.toBuffer()], program.programId);

  const addOrderArgs = {
    orderId: new anchor.BN(orderId),
    customer: customer.publicKey,
    customerName: 'Matt',
    items: [new anchor.BN(menuItemSku)],
    total: new anchor.BN(menuItemArgs.price),
    status: 0,
    createdAt: new anchor.BN(today.getDate()),
    updatedAt: null,
    bump: orderBump
  };

  it("Add Customer Order to Restaurant!", async () => {
    const customerAta = await getAssociatedTokenAddressSync(mint.publicKey, customer.publicKey, true)
    const restaurantAta = await getAssociatedTokenAddressSync(mint.publicKey, newRestaurant.publicKey, true)
    // Add your test here.
    const tx = await program.methods
      .restaurantAddOrder(addOrderArgs)
      .accountsPartial({
        order: orderPda,
        customer: customerPda,
        signer: customer.publicKey,
        currency: mint.publicKey,
        signerAta: customerAta,
        restaurantAta: restaurantAta,
        restaurant: newRestaurant.publicKey,
        instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      });
    // console.log("Your transaction signature", tx);
  });

});
