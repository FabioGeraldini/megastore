\# Decisões de projeto



\## Modelagem

\- Vértices: Cliente, Produto, Categoria

\- Arestas: Cliente-Produto (compra/avaliação), Produto-Categoria (pertence a)

\- Pesos: nota da avaliação ou quantidade comprada

\- Tipo: não direcionado e ponderado



\## Representação

\- Lista de adjacência: Vec<Vec<(usize, f32)>>

\- Motivo: grafo esparso; matriz custaria O(V²) de memória



\## Estruturas complementares

\- HashMap<u32, usize>: ID externo -> índice do vértice, acesso O(1)

\- HashSet: evitar duplicatas e marcar visitados

\- VecDeque: fila do BFS

\- BinaryHeap: top-K das recomendações



\## Algoritmo

\- BFS com profundidade limitada, partindo do cliente

\- cliente -> produtos comprados -> outros clientes -> produtos novos

\- Pontuação: soma dos pesos dos caminhos

