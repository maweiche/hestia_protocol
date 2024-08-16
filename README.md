# I am building
I am building Hestia, a dApp to allow restaurant owners to take their biz operations on-chain within 20 minutes. 

Issue: Small/Medium sized restaurants are forced to use multiple costly 3rd parties to operate (Square, Toast, etc.) 
Solution: Consolidate inventory management, CMS, Menu, and POS system into one on-chain program.
Benefits: Lower overhead, enhanced customer engagement with rewards, customers can pay card like normal or with crypto, online ordering instantly available
Target Market: Food Trucks, Cafes, anyone using an ipad POS to operate.
User Types: Restaurant Owner, Customer, Employee


# Target Market
Food Trucks, Cafes, anyone already using an ipad POS to operate.

# User Stories
 <!-- "As a [user], I want [functionality], so that [benefit]"  -->
 <!-- Identify the users involved. -->
 <!-- Focus on a single feature per story. -->
 <!-- Include "Acceptance Criteria". -->
 <!-- Ensure that each story can be tested. -->
 <!-- Order and prioritize them. -->
 <!-- Include non-functional requirements. -->


 <!-- 3 USER TYPES + STORIES -->
 <!-- RESTAURANT OWNER -->
  As a restaurant owner...
    1 - I want to take orders in store and display/update them from the back-of-house.
    1 - I want to add employees to my restaurant who can execute different functions based on their permissions.

    2 - I want to manage my inventory and have it update when an order from the menu is placed
    2 - I want to add/update/remove items from my menu
    2 - I want to offer rewards to be redeemed for free products and be able to airdrop them to users

    3 - I want to be able to pull reports for any given time

 <!-- CUSTOMER -->
  As a customer...
    1 - I want to pay card/crypto, i don't care if it's on the blockchain
    1 - I want to be able to place a mobile order
    1 - I want to be able to cancel my order

    2 - I want to receive rewards for my customer loyalty

 <!-- EMPLOYEE -->
  As an employee...
    3 - I want to clock in and out by scanning a qr code
    3 - I want to be able to display my qr code for tips

<!-- PLAN OF ACTION ( 3 weeks ) -->
    1. 
        - Finish Program for orders/inventory/customer loyalty points
        - Build in-store order UI and Back-of-House UI for incoming order display -> must have fn to update order status
        - Build UI to ready inventory to verify it updates when orders placed

    2.
        - Build mobile order UI -> display available restaurants -> display specific after selection
        - Build Owner UI to add/remove/airdrop rewards

    3. 
        - Build QR code display for clock-in/out
        - Build QR display ui for "who's on shift"
        - Build a report generating system



<!-- IDEAS DTR -->
    1. Employee NFT instead of PDA, allow for employee to verify employment by scanning at other locations -> discounts etc.

    2. Need a better inventory formula for tracking on menuitem, rn it is just sku, but need sku/amount -> ingredient pda? 

    3. Set tax rates on Restaurant state? 

    4. Payroll from Restaurant, daily settlemen?