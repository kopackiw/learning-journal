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

### 2. Public key cryptography

_Crypto_ in cryptocurrency stands for cryptography!

<details>
   <summary>Symmetric cryptography</summary>

#### Symmetric cryptography

Symmetric cryptography assume, that two users which want to communicate, encrypt their data using encryption algorithm with shared key K. After receiving an encrypted message, they would receive an original text with decryption algorithm with the shared key K.

##### Authentication

It is possible to enable authentication method to prove data integrity send over insecure channel. One approach is to use a message authentication code (MAC or a tag) which is generated based on key K and message M and are sent altogether with message. If the receiver run the same tag generation algorithm with K and M and obtain a different tag, it implies that message was malformed.

##### Problems with this approach

If both keys should be shared and known only to these two users, how they can establish a secure channel where they can agree on the keys? The question is more important in the Internet era, where these end users may not know each one.

</details>

<details>
   <summary>Asymmetric cryptography</summary>

#### Asymmetric cryptography

Asymmetric cryptography introduces private and public keys. Public keys allow to encrypt the message - anyone who sends the message can do it. To decrypt it, one must hold a private keys matched for the used public key. It is a one person well hidden secret.

</details>

<details>
   <summary>Summary</summary>

#### Summary

Although asymmetric encryption is more secure than a symmetric one, it is also more inefficient. Thus, a hybrid approach exist: use asymmetric cryptography to establish a secure channel of communication, share keys for symmetric cryptography and then use them for further communicating through an insecure channel.

</details>

### 3. Intro to hashing

To the cash through a hash :D

<details>
   <summary>What is hashing?</summary>

#### What is hashing?

A one-way function is a function _y = f(x)_ where, given _y_, it is computationally infeasible to calculate _x_, but given _x_, it is possible to calculate _y_.

Hash is an example of a one-way function\* that take an some data and returns an fixed-length unique\*\* value representing it.

\* - no one can prove that specific algorithm is one-way, it's rather the trust convention
\*\* - may not be unique, but it is _highly_ unlikely

</details>

<details>
   <summary>Collisions</summary>

#### Collisions

As mention in the subchapter above, two different strings can be hashed to the same result. This situation is called a collision. Although a hashing function which is collision-free does not exists, it is extremely hard to "produce" this situation.

</details>

<details>
   <summary>Good vs bad hashing</summary>

#### Good vs bad hashing

There are some rules that good hashing function must meet:

1. collision-resistant - summed up above
2. hiding - there is no way to "reverse engineering" the algorithm (no clear correspondence with change in the message to a change in the output)
3. puzzle friendliness - no shortcuts, only brute force attacks are possible

</details>
