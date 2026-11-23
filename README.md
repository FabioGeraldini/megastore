\# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos



Projeto da disciplina \*\*Data Structure Strategy and Implementation\*\* (UniFECAF).

Autor: Fabio Geraldini



\## Objetivo



A MegaStore vende milhões de produtos, mas recomenda apenas os mais vendidos ou itens da mesma categoria. O ConectaStore modela clientes, produtos e categorias como um \*\*grafo\*\* e percorre as conexões de compra para recomendar produtos relevantes a um cliente ou a partir de um produto.



\## Como funciona



\- \*\*Vértices:\*\* Cliente, Produto e Categoria.

\- \*\*Arestas:\*\* Cliente–Produto (compra/avaliação, peso = nota) e Produto–Categoria (peso 1.0).

\- \*\*Tipo de grafo:\*\* não direcionado e ponderado.

\- \*\*Recomendação para um cliente:\*\* BFS com profundidade limitada a 3:

&#x20; cliente → produtos que comprou → outros clientes que compraram os mesmos produtos → produtos desses clientes que ele ainda não possui.

&#x20; A pontuação de cada candidato soma, sobre os caminhos encontrados, o produto dos pesos das arestas.

\- \*\*Recomendação a partir de um produto ("quem comprou isto também comprou"):\*\* produto → clientes que o compraram → outros produtos desses clientes.

\- Produtos já comprados são excluídos e não há recomendações duplicadas. O resultado é o top-K ordenado por pontuação.



\## Tecnologias e estruturas de dados



\- Rust (edição 2021), sem dependências externas.



| Estrutura | Uso | Motivo |

|---|---|---|

| `Vec<Vec<(usize, f32)>>` | Lista de adjacência | Grafo esparso: espaço O(V + E), contra O(V²) da matriz |

| `HashMap<u32, Product>` | Catálogo de produtos | Acesso por ID em O(1) médio |

| `HashMap<(NodeKind, u32), usize>` | ID externo → índice do vértice | A chave inclui o tipo, pois cliente 1 e produto 1 são vértices distintos |

| `HashSet` | Visitados e produtos já possuídos | Teste de pertinência em O(1) médio |

| `VecDeque` | Fila do BFS | Inserção e remoção em O(1) |

| `BinaryHeap` (min-heap de tamanho K) | Top-K | O(m log K) em vez de ordenar todos os m candidatos |



\## Estrutura do projeto



```

megastore/

├── src/

│   ├── main.rs          # demonstração e benchmark

│   ├── lib.rs           # expõe os módulos

│   ├── product.rs       # struct Product

│   ├── catalog.rs       # cadastro e consulta de produtos (HashMap)

│   ├── graph.rs         # grafo com lista de adjacência

│   ├── recommender.rs   # BFS limitado, pontuação e top-K

│   └── generator.rs     # gerador de dados sintéticos

├── tests/

│   └── integration\_test.rs

├── docs/

│   ├── decisoes.md

│   └── resultados.md

├── Cargo.toml

└── README.md

```



\## Compilação e execução



Requer o Rust instalado (https://rustup.rs).



```bash

git clone https://github.com/FabioGeraldini/megastore.git

cd megastore

cargo build --release

cargo run --release             # demonstração + benchmark

cargo run --release -- demo     # somente a demonstração

cargo run --release -- bench    # somente o benchmark

```



\## Testes



```bash

cargo test

```



São 19 testes: 13 unitários (catálogo, grafo e recomendador) e 6 de integração (fluxo completo, grafos sintéticos, ausência de duplicatas, ordenação e determinismo do gerador).



\## Exemplo de uso



Saída de `cargo run --release -- demo`:



```

Recomendacoes para Ana:

&#x20; - Teclado (score 100.0)

Recomendacoes para Bruno:

&#x20; - Mouse (score 80.0)

Recomendacoes para Carla:

&#x20; (nenhuma: sem clientes com compras em comum)



Quem comprou Notebook tambem comprou:

&#x20; - Mouse (score 20.0)

&#x20; - Teclado (score 20.0)

```



Ana e Bruno compraram o Notebook. Como Bruno também comprou o Teclado, ele é recomendado à Ana. Carla não compartilha produtos com ninguém, então não recebe recomendações (limitação conhecida de "cold start").



\## Arquitetura



1\. `Catalog` guarda os dados dos produtos (nome, categoria, preço), indexados por ID.

2\. `Graph` guarda apenas as conexões. Cada vértice tem um índice numérico, e a lista de adjacência é indexada por ele.

3\. `recommender` percorre o `Graph` e devolve IDs de produtos com pontuação. O `Catalog` traduz esses IDs em dados legíveis.



Separar dados do produto e conexões mantém o grafo leve e permite evoluir cada parte de forma independente.



\## Resultados de desempenho



Executado com `cargo run --release -- bench`, em \[Ryzen 7 7800X3D] com \[32] GB de RAM, Windows. Cada cliente do gerador compra 8 produtos; o catálogo tem 50 categorias; os tempos de recomendação são a média de 100 consultas.



| Produtos | Clientes | Vértices | Arestas | Construção | Rec. cliente (média) | Rec. produto (média) |

|---|---|---|---|---|---|---|

| 1.000 | 500 | 1.550 | 5.000 | 528,50 µs | 21,71 µs | 1,72 µs |

| 10.000 | 5.000 | 15.050 | 50.000 | 5,42 ms | 19,48 µs | 1,80 µs |

| 100.000 | 50.000 | 150.050 | 500.000 | 58,96 ms | 24,61 µs | 2,17 µs |

| 1.000.000 | 500.000 | 1.500.050 | 5.000.000 | 1,13 s | 45,79 µs | 3,97 µs |



A construção cresce de forma aproximadamente linear com o número de arestas. O tempo de recomendação cresce pouco, pois o BFS é limitado em profundidade e só explora a vizinhança do cliente. Nos dados sintéticos essa vizinhança é pequena e uniforme; em dados reais, produtos muito populares aumentam o custo, e o pior caso é O(V + E).



\## Complexidade



\- Espaço: O(V + E).

\- Acesso por ID: O(1) médio.

\- Recomendação: O(soma dos graus dos vértices explorados) ≤ O(V + E), mais O(m log K) para o top-K.



\## Vídeo pitch



\[COLE AQUI O LINK DO YOUTUBE]

