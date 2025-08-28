# WAV Fingerprint

Um algoritmo de fingerprint de áudio inspirado no Shazam, desenvolvido em Rust para identificar músicas através de suas características acústicas únicas.

## 🎵 Sobre o Projeto

Este projeto implementa um sistema de fingerprint de áudio que permite identificar músicas através da análise de suas características espectrais. O algoritmo funciona de forma similar ao Shazam, criando uma "impressão digital" única para cada trecho de áudio que pode ser usada para identificação rápida e precisa.

## 🚀 Funcionalidades

- **Leitura de arquivos WAV**: Suporte completo para arquivos de áudio no formato WAV
- **Processamento de áudio**: Normalização e preparação dos dados de áudio
- **Análise espectral**: Extração de características frequenciais do áudio
- **Geração de fingerprints**: Criação de identificadores únicos baseados em características acústicas
- **Identificação de músicas**: Sistema de matching para encontrar correspondências


## 🧠 Como Funciona

### 1. Leitura do Áudio
O sistema lê arquivos WAV e normaliza os dados de áudio.

### 2. Análise Espectral
Aplica transformações de Fourier para converter o sinal temporal em domínio frequencial.

### 3. Extração de Características
Identifica picos de frequência e suas relações temporais para criar características únicas.

### 4. Geração do Fingerprint
Combina as características extraídas em um hash único que representa o trecho de áudio.

### 5. Matching
Compara fingerprints para encontrar correspondências e identificar músicas, utilizando hashmaps para ter uma melhor performance

## 🚧 Status do Desenvolvimento

- [x] Leitura de arquivos WAV
- [x] Normalização de áudio
- [ ] Análise espectral (FFT)
- [ ] Extração de características
- [ ] Algoritmo de fingerprint
- [ ] Sistema de matching
- [ ] Interface de linha de comando
- [ ] Documentação da API
