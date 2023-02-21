# DevRev’s Expert Answers in a Flash Improving Domain-Specific QA

<!-- DevRev’s Expert Answers in a Flash Improving Domain-Specific QA -->
<img src="https://socialify.git.ci/siddharthkhincha/Inter-IIT-11-Devrev/image?description=1&font=Raleway&language=1&name=1&owner=1&pattern=Signal&stargazers=1&theme=Light">

---

## Table of Contents

- [Introduction](#introduction)
- [Problem Statement](#problem-statement)
- [Dataset](#dataset)
- [Approach](#approach)
- [Results](#results)
- [Conclusion](#conclusion)

## Introduction

This is a solution for DevRev PS at Inter IIT Technical Meet 11.0 aiming to minimize the number of communication which reaches the support agent and let the system handle it initially while providing an automated response with some suggestions.

Given a query, we developed a system which provides a Knowledge
Base article (KB article) to the given user query. For example, if the user comes up with a query about how payment API works, our system should be able to pull up a related API documentation which would be able to answer the query. And also highlight a
part of the document which is the potential answer. Getting this done right for different distribution of documents makes it even more complex.

We optimize the pipeline for
performance, latency, and resource usage. The
availability of diverse knowledge bases makes this
task challenging. We propose novel methods to
handle FAQs, generate synthetic queries, model
fine-tuning, retrieve candidate sentences for answer matching and improve runtime efficiency

## Problem Statement

### Task 1

Given a question and a set of paragraphs, predict if the question can be answered with the given paragraphs. If yes, return the paragraph that answers the question. Each question and paragraph is associated with a specific theme. This could be “Sports”, “English” or “Mathematics” etc. A question of a theme can be
answered by one of the paragraphs in that theme.

### Task 2

For the given questions, also predict the exact answer from the
predicted paragraph. Predict the start_index and the answer_text field for the given question. Note: Both the tasks will be marked individually. However, to perform better in Task 2, your model needs to perform better in Task 1.

## Dataset

The training data provided was a subset of SQuAD2.0 dataset.
The dataset contains the following fields:

1. Question
2. Theme
3. Paragraph
4. Answer_possible
5. Answer_text
6. Answer_start

## Approach

<img src = "pipeline\final-pipeline.jpg">

## Results

## Conclusion
