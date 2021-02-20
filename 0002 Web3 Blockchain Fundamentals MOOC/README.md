# Web3 Blockchain Fundamentals MOOC

There is a crypto-currency hype nowadays. But this is only a one example usage of technology called blockchain. The expectation is that it will be use heavily in the future, so why not learn it today?

## Goal of the project

1. To gain knowledge how blockchain works, what its pros and cons and why it stands out.

## Project resources

1. Web3 Foundation , _Web3 Blockchain Fundamentals MOOC_
   - view [the lecture playlist](https://www.youtube.com/playlist?list=PLxVihxZC42nF_MCN9PTvZMIifRjx9cZ2J) on YouTube

## Project journey

### 1. History of blockchain technology

Although blockchain is not strictly related with money, the concept will be based on money as it is easy to visualize.

<details>
   <summary>Cash vs debt</summary>

#### Cash vs debt

In order to create a basic economy, we **do not need** to use cash explicitly. A more "primitive" version of determining the wealth is to track the personal debt itself. You own everybody - you are "poor"; everybody own you - you are "rich". We use cash on a daily basis, because it has more advantages over debt-base system.

|      | cash                                                                                                                  | debt                                                                                                                                                       |
| ---- | --------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| pros | 1. Anonymity - you finalize the transaction and no leave no trace.                                                    | 1. Simple mechanism - no need for external goods (coins, gold etc.).                                                                                       |
| cons | 1. One may end up with no confirmation of the transaction. <br> 2. Need consensus what can play as a mean of payment. | 1. Lack of anonymity - the debt owner (seller) as well as buyer must be exactly listed. <br> 2. Third party authority must manage and verify transactions. |

</details>

<details>
   <summary>Problems with currency</summary>

#### Problems with currency

##### Double-spending issue

The currency system must solve the problem of double-spending issue. It's a situation when the same piece of money is used to pay in two different transactions.

##### Central authority issue

Following the rule _trust nobody_, the lack of central authority gives more power (as well as responsibilities) to users of the currency.

##### Unlimited resources issue

As there are more money being added to the pool, the overall amount increases, so overall purchasing power (for the same amount) decreases. If money are being added and added, the currency is worthless.

</details>

<details>
   <summary>How does Bitcoin solve those problems above?</summary>

#### How does Bitcoin solve those problems above?

##### No work - no money

In real word we are paid for doing a job. Why not apply this rule for work that computers do? In Bitcoin world, the computer is being paid for its computational work done during processing a block (list) of transactions.Instead of a single transaction, the block is object being processed, as it has good level of granularity (and also it is more efficient).

##### Is those computations "useful"?

I was shocked when I realize that the computation is not "meaningful" in terms of solving a general problem which need a huge amount of power (e.g. looking for primes). Then I realize that these computations must be linked with transactions packed in the block a computer is trying to produce. It would be very interesting if there will be some way to linked this two approaches.

##### Ledgers

To enforce block order, each of them must point to a previous one ("have a parent"). This create a chain of block (hence "blockchain" 🙊).

##### Decentralization

With a consensus that the largest proof of work is a leading ledger, there is no need for central authority to manage and approve transactions.

##### Bitcoin halving

Bitcoin is limited to only 21 millions of coins and has a mechanism of halving (every 210,000 block mined the reward is cut in half). When all coins will be mined, the miners will be paid from transaction fees only.

</details>