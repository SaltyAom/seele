# Seele
[Hifumin](https://hifumin.app)'s API or Hentai GraphQL API implemented in Rust

<img src=https://user-images.githubusercontent.com/35027979/177157769-0195c054-cdd4-444b-98a4-3dcff6abc5ba.JPG alt="Bronya Onee Chan" width=420 />

## What is
Seele is GraphQL nHentai Reverse Proxy implemented in Rust.

Current API is running [at api.hifumin.app](https://api.hifumin.app), and deployed on [deploys.app](https://deploys.app).

## Why
The goal is to be as fast as possible, and globally accessible.

That's why I published the [deployment](https://api.hifumin.app) as free-to-use, without CORs restriction.

Average response time is between 50-90 ms including Roundtrip tested in SEA (South East Asia) area.

## Channel
Seele has 3 API channels:
- nHentai
- Hifumin
- Hifumin First (default)

### nHentai
nHentai channel will be directly use data fetch from nHentai API.

- Data is fresh
- Slower than Hifumin (expected: 300-400ms)

### Hifumin
Hifumin will cache nHentai data, and update every 12 hours.

- Data is not as fresh as nHentai
- Some new H may not be available at time
- Fast (expected: < 90ms)

### Hifumin First
`Hifumin First` will use data from `Hifumin` first, then if not available will fallback to `nHentai`.

- Has both strength of Hifumin, and nHentai
- 100% data integrity

## Roadmap
Done:
- [x] NHentai
    - [X] nhql remap
    - [X] search engine
    - [X] related

On consideration:
- [ ] e-hentai.org
- [ ] Fakku
    - [ ] Implement custom scrapper as public API went shutdown

## QA
<img width=400 src="https://user-images.githubusercontent.com/35027979/133552450-0dd6e24e-6c80-4658-be9b-72fd8308efbd.png" alt="Elaina eating borgor" />

- Why?
    - Because I can
- Will you deploy this?
    - [Yes](https://api.hifumin.app)
- Will it have CORS restriction?
    - No, that's the goal
- Will this be free to use?
    - Open-source with MIT License, so yes, just keep the License then all good
    - Since I have to bear the deployment cost, please consider about donation at [my Kofi page](https://ko-fi.com/saltyaom)
- Why Rust?
    - If any software existed, it will be rewritten in Rust
- Can I request for more API services?
    - Maybe, but you can request one in [discussion](https://github.com/SaltyAom/seele/discussions)
