# Test Page
<!-- #SQUARK live!
| title = Testing Squarkdown’s Performance at Scale
| description = How does Squarkdown hold up for thousands of files?
| tags = tests / benches
| date = 2026 October
---
| field1 = 1
| field2 = 2
| field3 = 3
| field4 = 4
| field5 = 5
| field6 = 6
| field7 = 7
| field8 = 8
| field9 = 9
| field10 = 1 / 10
| field11 = 2 / 11
| field12 = 3 / 12
| field13 = 4 / 13
| field14 = 5 / 14
| field15 = 6 / 15
| field16 = 7 / 16
| field17 = 8 / 17
| field18 = 9 / 18
| field19 = 10 / 19
| field20 = 11 / 20
| field21 = 12 / 21
| field22 = 13 / 22
| field23 = 14 / 23
| field24 = 15 / 24
-->

The aim here is to test how Squarkdown’s performance holds up for colossal input sizes.

<!-- #SQUARK slash? -->

This scale is unrealistic, but scale is how you track the effects of micro-optimisations!

<!-- #SQUARK slash. -->

##

> [!Important]
> This page aims to use *as much **nested** [`Markdown` ~~syntax~~](https://sup2point0.github.io/stranger-quarkdown) as possible*, to push the limits of context tracking.

To do that, we’ll need to have a healthy mix of <!-- interjections --> varied syntax and

<!-- #SQUARK leave? -->

wacky syntax that’ll require $\text{potentially a lot}$ parsing and processing, especially things like

<!-- #SQUARK leave. -->

lines with <!-- interleaved --> comments

or terminating <!-- comments -->

Comments that

<!--

span
multiple
lines

with

<!--

weird syntax

-->

And then we also want stuff like:

### Large Code Blocks

```rs
// == PARSE == //
log::is!("finding files to squarkup...");

for filepath in resolver::resolve_files(&config) {
  site_data.stats.checked_files += 1;

  catch! {
    let filepath = filepath?;
    let r = parser::parse(&filepath, &config)?;
    
    if let Some(page) = r {
      site_data.add_page(page);

      let (folder, file) = utils::split_path_from(filepath, &config.paths.root);
      log::info!("found active file: {folder}/{GREY1}{file}");
    }
  }.or_else(|e| e.depends(&config))?;
}
```

### Large LaTeX Blocks

```math
```math
\int
  \left[
    \frac
      { \cos{x} + x \sin{x} }
      { \cos{x} }
  \right]
  \left[
    \frac
      { x^5 \sec(x)^6 }
      { 2 \pi }
    \exp\left(
      \frac{4}{3} \pi x^3 \int_{0}^{\infin}
        \frac
          { \ln{y} }
          { 1 + y^2 }
      \ dy
    \right)
    +
    \frac
      { x^3 \sec(x)^4 }
      { \pi }
    \exp\left(
      4 \pi x^2 \int_{\infin}^{0}
        \frac
          { \ln{y} }
          { 1 + y^2 }
      \ dy
    \right)
  \right]
\ dx
```

And we also want each file to be pretty long to test real large input, so here goes:

1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog
1. The quick brown fox jumps over the lazy dog

No no, not enough.

- A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.
  - A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.
    - A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.
	    - A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.
        - A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.
          - A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.

A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.

A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.

A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.

A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.

A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.

*A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.*

**A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.**

__A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.__

~~A common misconception is that between attack speed and attack delay, which are in fact direct inverses of each other. Attack delay refers to the time delay between each attack, often excluding animations, and so the lower, the stronger (ignoring context and game-specific nuances). For example, consider 2 weapons with different cooldowns: 1 shall have a delay of 0.3s between each attack, while the other 0.6s. Evidently the weapon with 0.3s fires faster, and intuitively it follows that it fires twice as fast as the other. This leads nicely on to attack speed, which is generally the preferred metric. This describes the number of attacks over a given duration, often a second, and so is usually described in attacks per second. Note diving 1 by attack delay yields the attack speed – in other words, taking the reciprocal – as it essentially corresponds to the question of “how many times can I fit this cooldown delay into 1 second?” In this way, an interesting relationship between attack speed and delay is revealed: they are directly inversely proportional to each other, where k in xy = k is precisely 1 (given that their time units are equivalent). A slightly more confusing but nonetheless critical aspect to wrap our heads around, is that of increasing and decreasing attack rate. A linear increase in attack speed corresponds to an exponential decrease in attack delay, and vice versa. For this, consider decreasing attack delay from 1s to 0.9s, compared to decreasing from 0.2s to 0.1s. The latter, relatively, is a 50% drop, while the former is only 10%. Similarly, increasing from 1 attack per second to 2 is a significant 100% increase, while 10 to 11 is a mere 10%. Plotting these relationships onto a graph, with the x axis being increasing attack speed, we see that attack delay forms a reciprocal curve. Swapping the x axis for decreasing attack delay, attack speed will form an exponential curve. Hence, when considering the value of a particular upgrade, it is always essential to remember the relative increase in strength, as opposed to the absolute change.~~

1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. even more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more
1. still more

- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()
- [a link]()

- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)
- [another link](https://sup2point0.github.io)

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle

We could get more machine gun psystyle


damn I wonder how compression on this will be


Let’s add some entropy

U3F1YXJrZG93biBpcyBhd2Vzb21lIEkgbG92ZSBTcXVhcmtkb3duIFNxdWFya2Rvd24gaXMgYXdlc29tZQ==
