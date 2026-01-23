# Restaurante-rust

## Motivação do projeto

Este pequeno projeto foi inspirado em uma atividade realizada na cadeira de Programação Imperativa, onde se era necessário montar um código C que operasse um sistema de menu de restaurante.

O objetivo principal deste repositório é documentar meu processo de aprendizado em Rust. Ao portar um desafio da programação imperativa para uma linguagem moderna, busco explorar conceitos como segurança de memória, ownership e a robustez que o ecossistema Rust oferece.


## Ideias adcionadas

Uma das principais implementações que diferenciam o atual projeto feito em rust da atividade feita em C é o salvamento e carregamento automático dos dados.

Esta foi implementada por meio do framework Serde, que é capaz de gravar as informações de uma estrutura de dados em um arquivo json, que podem posteriormente serem carregadas.

Essa implementação trabalha para que o menu não seja perdido mesmo com um eventual encerramento do programa.


## Branches

main: Mantém a implementação original baseada em vetores (Vec). É a base estável do projeto, focada na lógica imperativa inicial.

hash-map: Branch experimental criada para otimizar a busca e manipulação dos itens. A transição de um vetor para um HashMap foi motivada pela busca de maior eficiência, reduzindo a complexidade de tempo de busca de $O(n)$ para $O(1)$ em média.
