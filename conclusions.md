# Current state of Kusama governance

## Problem definition
PolkaDAO is staying apart from Polkadot ecosystem and would like to be merged.

Things to be solved:
- Proper positioning of PolkaDAO to enter Polkadot ecosystem;
- How and where can PolkaDAO receive fundings within Polkadot ecosystem;
- How do PolkaDAO members receive funds for their contribution;

### Positioning
PolkaDAO is a broader audience community for Polkadot ecosystem.  
It is open and permissionless, focused on onboarding of new members and helping
them finding issues and topics to investigate, research, build and collaborate.  
**PolkaDAO is a connecting hub to Polkadot.**

On the other side there is an ambassador program with its community that seems
to be very similar to PolkaDAO. I'd expect that those communities might be
merged. Also I consider that goals of ambassador program is to set up local
communities and PolkaDAO might be like global online community.

One of the main goals of PolkaDAO is to grow the community around Polkadot
ecosystem.

### PolkaDAO funding
Since it initial creation PolkaDAO existed on Ethereum network as a DAOStack DAO
community funded with 12k worth of DAI. Right now the funds are low, but there's 
still some community in it, willing and looking forward onto further PolkaDAO
evolution. Let's find a way how PolkaDAO can receive funds.

First of all, probably, Kusama network might be more suitable to take on
PolkaDAO community, being a canary network, that is faster and more actively
updated than Polkadot itself. Though having real value KSM tokens don't have
any price expectations or general evaluation due to the initial token
distribution model.

The problem is that substrate framework is build from pallets that are compiled
into runtime that is being upgraded by voting in referenda. It's not that easy 
to integrate into the network right now as compared to Ethereum with smart
contract solution for example. But existing treasury system seems to be weak and
there are lots of doubts. We have an opportunity to find a way to integrate
PolkaDAO into Treasury right now as it is being constantly refactored and
reworked at the moment.

Another option for PolkaDAO is to use smart contract system that will hopefully
launch contracts pallet sometime soon. This way we are free to build the
structure we want given the fact that we receive funds into our smart contract
address.

#### Treasury modification proposal
##### Existing problems
There are several issues with existing ways to receive funds from the treasury.
Basically we have 3 parties dependent on the success and functioning of the
treasury distribution: the council (collective currently responsible on taking
decisions for funds distribution), beneficiaries (ones who receive funds from
the treasury), passive stakeholders (general community interested in proper
funds distribution).

After reviewing existing models and following up chats and discussions I see
following issues with the treasury implementation:

**Proposals**  
- Council is not incentivized directly and probably having main focus on
  personal project; no incentives to vote or research proposals
- Otherwise council members don't have expertise on the subject and don't know
  how to vote
- There was a role of prime voter introduced to allow council members to soft
  approve or soft reject his decision. Prime voter (or Raul) is expected to ping
  other council members. Raul created a special email newsletter, is reaching
  council members via DMs and Kusama Direction channel
- If prime member is digesting and proposing proposal information on his own, he
  might be biased and use his role in his interest to get soft voting from voters
- Passing voting is not serving the goals of council to represent passive stake
  holders
- There are still issues happening with KSM price estimation with expected cost
  estimations. Until now the price of KSM tokens was growing but proposers were
  normally taking risks of the volatility, in some cases of rapid price growth
  council was suggesting to reduce KSM reward accordingly that always was not
  accepted by proposers
- Council division on the usage of treasury. There are couple of members that
  are very welcoming and willing to distribute funds in the treasury for small
  proposals, others are more concerned to distribute funds in the proper hands.
  Decisions made are being selfish, though the funds do belong to the network
- If council does not like proposal and are tending to reject it, depositor will
  loose his stake, though he might not be spamming  
 
**Tips**
- How would one estimate the size of the tip given decentralized location of
  council members
- Members are biased by other members tip value

##### Random thoughts
- Reviewing proposals, submissions, contributions is time and energy consuming.
  Should actually collective dealing with treasury be devoted to voting and
  receive rewards for doing such a work??
- Organize elections for this collective by general audience with election
  programs, focus, community vision, etc
- Keep an updated list of tasks and provide community the ability to request 
  to work on them (do we need reputation here?)

##### Solution relevant to PolkaDAO
Existing system is expected to work well for technical contributors with known
cost estimation or random contributions found by community that was not the
subject for any reward. But for many other cases existing system does not suit
and it will probably have issues with scalability.

This solution needs changes to be done in the treasury pallet, but expectedly
this structure will allow the system to scale, to involve much more
participation from the community and will be able to bring PolkaDAO to Polkadot.

- Instead of having burn rate to put a pressure and create deflation in the
  system, introduce community work tipping rate. Those funds are going into the
  accounts of the contributors of the network.
- Council (or treasury dealing collective) defines separate areas of work
  (marketing, tutorials, community tools, workshops) and distribute funds
  proportionally each period into those areas by voting with their preferences; 
- Areas/Circles/Domains are having separate leaders, proven experts in the
  specified areas willing to devote their time for the good of the specified
  area of development of Polkadot ecosystem; they are elected by general
  community of token holders;
- Those leaders report back to treasury collective with the progress and status
  of the area; that report is altering future period distribution of funds
- Funds are being redistributed within the area by the impact made or attention
  received from the members of the same area
- PolkaDAO is one of these circles

As soon as the quality and quantity of submissions is increasing council members
can in return increase the rate of community work tipping and thus the amount of
funds being distributed to the working areas.

### PolkaDAO (area) internal distribution of funds
Areas are open for all kind of requests. We should not be slashing people for
making proposals, but rather simply ignoring them.

There are two ways to distribute funds:
1) Members produce some beneficial work for their area, promote the work done
and receive attention from their community (could be likes, upvotes, whatever).
By the end of the period contributors receive respective part of funds of the
area. Importantly here contributions are having context and receive impressions
throughout whole period
2) Same way members do contribute throughout the period, but instead of
attracting attention by the end of the period community defines its preferences
over impact of the contributions in pairwise voting. Here I recall and assume
Colony's [BudgetBox] to be implemented on Polkadot.

Otherwise we can implement simpler solution similar to current DAOstack
implementation. We can remove many things and build something small and simple
enough.

Generally community is supportive and willing to support whatever kind of
initiatives and there's no reason to vote on something that you soft approve.
We can have similar prime voter defining general audience status on the
proposal. If there's still somebody against the proposal he can vote nay on the
proposal and trigger whole community voting. There should be some reward defined
for voting and punishment for someone triggering the appeal if not supported.


### Conclusions
Logic to have areas of work would need to be implemented in the runtime.
In the case we cannot do this, in the future PolkaDAO smart contract might
receive funds from treasury tip or proposal mechanics and implemented relevant
logic using smart contracts.

[BudgetBox]:https://colony.io/budgetbox.pdf
