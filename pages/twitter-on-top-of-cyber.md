- Yesterday become apparent that Twitter as we know it ends. The inability to bring cryptographic assurance was lead to the program's behaviour, which we, as a society, must not accept. There are tons of ways to implement tweets on top of a blockchain. One of which is using the payload of a transaction for content and follow intentions. But I would argue that implementation on top content-addressable knowledge graph has drastic advantages.
## Benefits
- The blockchain is not bloated as only content addresses are being stored.
- Natural sources of trust emerge in the form of *computed* rank onchain. If conventional blockchain is being used, ranks computation and storage have to be solved using L2 token, which is not dev-friendly.
- Cyberlinks offer an affordable cognitive model of consumption because you have not to pay per bytes fees.
- Transactional semantics was created with value transfer in mind, and not around content interactions. Hence this way is just not as powerful as cyberlinks are.
- All benefits of [Software 2.0](https://medium.com/@karpathy/software-2-0-a64152b37c35) are also inherited
  
  Okay, let us showcase how easy the implementation is.
## Avatars

It is absolutely necessary to let agents have avatars. Let them do it by making the following cyberlink:

```
<my><avatar><signer_address><picture_cid>
```
## Names

There is no easy way to control the name's uniqueness in the naive knowledge graph as the only way is to depend on the timestamp of the first mention. This method has a disadvantage: you need to perform an expensive query every time you want to know about the address's correct handle. I like the idea of deterministic urbit-like sigils, but let us accept that names are unnecessary for our task. Let them emerge naturally.
## Following

Let agents follow any address they wish:

```
<i><follow><signer_address><followee_address>
```
## Feeds

It is trivial using the semantics mentioned above to query feeds using some indexed structure, such as [cyberindex](https://github.com/cybercongress/cyberindex).
## Summary

We define the most straightforward way of creating twitter on top of cyber. The model can be significantly improved by adding likes, reposts, following content addresses, and more.