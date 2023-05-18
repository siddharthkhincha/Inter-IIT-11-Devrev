<!-- # DevRev’s Expert Answers in a Flash Improving Domain-Specific QA -->

<!-- DevRev’s Expert Answers in a Flash Improving Domain-Specific QA -->
<img src="https://socialify.git.ci/siddharthkhincha/Inter-IIT-11-Devrev/image?description=1&font=KoHo&name=1&pattern=Solid&stargazers=1&theme=Dark">

---

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href = "#introduction">Introduction</a></li>
    <li><a href = "#problem-statement">Problem Statement</a></li>
    <li><a href = "#dataset">Dataset</a></li>
    <li><a href = "#our-approach">Our Approach</a></li>
    <li><a href = "#key-points">Key Points</a></li>
    <li><a href = "#contributors">Contributors</a></li>
  </ol>
  </summary>
</details>

---

## Introduction 

This is a solution for DevRev's Problem Statement at the **11th Inter IIT TechMeet** aiming to minimize the number of communication which reaches the support agent and let the system handle it initially while providing an automated response with some suggestions.

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

<!-- 1. Question
2. Theme
3. Paragraph
4. Answer_possible
5. Answer_text
6. Answer_start -->
1. **Question** - Question for which answer is to be found
2. **Theme** - Name of the domain this question & paragraph belongs to.
3. **Paragraph** -  Paragraph from the mentioned theme which may contain the answer
4. **Answer_possible** -  If the answer is possible from the given paragraph
5. **Answer_text** - Answers from the given paragraph
6. **Answer_start** - Index position from where the answer starts

```
Theme,Paragraph,Question,Answer_possible,Answer_text,Answer_start
Beyoncé,"Beyoncé Giselle Knowles-Carter (/biːˈjɒnseɪ/ bee-YON-say) (born September 4, 1981) is an American singer, songwriter, record producer and actress. Born and raised in Houston, Texas, she performed in various singing and dancing competitions as a child, and rose to fame in the late 1990s as lead singer of R&B girl-group Destiny's Child. Managed by her father, Mathew Knowles, the group became one of the world's best-selling girl groups of all time. Their hiatus saw the release of Beyoncé's debut album, Dangerously in Love (2003), which established her as a solo artist worldwide, earned five Grammy Awards and featured the Billboard Hot 100 number-one singles ""Crazy in Love"" and ""Baby Boy"".",When did Beyonce leave Destiny's Child and become a solo singer?,True,['2003'],[526]
Military_history_of_the_United_States,"In January 2002, the U.S. sent more than 1,200 troops (later raised to 2,000) to assist the Armed Forces of the Philippines in combating terrorist groups linked to al-Qaida, such as Abu Sayyaf, under Operation Enduring Freedom - Philippines. Operations have taken place mostly in the Sulu Archipelago, where terrorists and other groups are active. The majority of troops provide logistics. However, there are special forces troops that are training and assisting in combat operations against the terrorist groups.",Terrorists in the Philippines were linked to what larger terrorist organization?,True,['al-Qaida'],[164]
Steven_Spielberg,"His next theatrical release in that same year was the World War II film Saving Private Ryan, about a group of U.S. soldiers led by Capt. Miller (Tom Hanks) sent to bring home a paratrooper whose three older brothers were killed in the same twenty-four hours, June 5–6, of the Normandy landing. The film was a huge box office success, grossing over $481 million worldwide and was the biggest film of the year at the North American box office (worldwide it made second place after Michael Bay's Armageddon). Spielberg won his second Academy Award for his direction. The film's graphic, realistic depiction of combat violence influenced later war films such as Black Hawk Down and Enemy at the Gates. The film was also the first major hit for DreamWorks, which co-produced the film with Paramount Pictures (as such, it was Spielberg's first release from the latter that was not part of the Indiana Jones series). Later, Spielberg and Tom Hanks produced a TV mini-series based on Stephen Ambrose's book Band of Brothers. The ten-part HBO mini-series follows Easy Company of the 101st Airborne Division's 506th Parachute Infantry Regiment. The series won a number of awards at the Golden Globes and the Emmys.",Which studio produced Armageddon?,False,[],[]
Religion_in_ancient_Rome,"Lucan depicts Sextus Pompeius, the doomed son of Pompey the Great, as convinced ""the gods of heaven knew too little"" and awaiting the Battle of Pharsalus by consulting with the Thessalian witch Erichtho, who practices necromancy and inhabits deserted graves, feeding on rotting corpses. Erichtho, it is said, can arrest ""the rotation of the heavens and the flow of rivers"" and make ""austere old men blaze with illicit passions"". She and her clients are portrayed as undermining the natural order of gods, mankind and destiny. A female foreigner from Thessaly, notorious for witchcraft, Erichtho is the stereotypical witch of Latin literature, along with Horace's Canidia.",How was Erichtho portrayed?,True,['stereotypical witch'],[602]
Matter,"The term ""matter"" is used throughout physics in a bewildering variety of contexts: for example, one refers to ""condensed matter physics"", ""elementary matter"", ""partonic"" matter, ""dark"" matter, ""anti""-matter, ""strange"" matter, and ""nuclear"" matter. In discussions of matter and antimatter, normal matter has been referred to by Alfvén as koinomatter (Gk. common matter). It is fair to say that in physics, there is no broad consensus as to a general definition of matter, and the term ""matter"" usually is used in conjunction with a specifying modifier.",What field of study has a variety of unusual contexts?,False,[],[]
```

## Our Approach

<img src = "pipeline\final-pipeline.jpg">

### Pipeline Overview

Our final pipeline for the given task includes the following
parts:

1. Previously Answered Questions
    * Fuzzy match with paraphrased questions
2. Sentence Retrieval
    * Retrieval of Top K (dynamically chosen) sentences to form the context for the question answering model.
3. Question Answering
    * Question-Answering model inference on the retrieved context.

## Key Points

<ul>
<li><h3>Sentence Based Retrieval</h3></li> 
<li><h3><a href = "https://github.com/SpyzzVVarun/question_generation">Question Generation using Transformers</a></h3></li><br><br>
<div markdown = "1">
- [ ] Insert Wide Image of QGen Pipeline
</div>
  <br><br>
<li><h3>Unsupervised Training of Retrievers Using GenQ</h3></li>
  <ol>
  <li>Generate queries for pre-existing but unlabeled passages: Creating (query, passage) pairs.</li>
  <li>Fine-tune a bi-encoder model using these (query, passage) pairs and Multiple Negatives Ranking (MNR) loss.</li>
  </ol>
  <br><br>
  <img src = "https://d33wubrfki0l68.cloudfront.net/072f1e19b9b0e0271da6946d46095b5891db6c2f/adf5a/images/genq-1.jpg" height = "250">
  <br><br>
  
  <li><h3>Paraphrasing Questions Using T5-Transformer</h3></li> 
  <ol>
    <li>Generated paraphrases of the questions given in the data using T5 Paraphraser.</li>
    <li>Used Fuzzy Match Ratio to find out the similarity between incoming question and the paraphrased questions.</li>
    <li>Used <a href = "https://github.com/Ki6an/fastT5">fastT5</a> library to reduce the runtime.<br><br>
  - [ ] Add Other Key Points

## Contributors

- [@SpyzzVVarun](https://github.com/SpyzzVVarun)
- [@amish1706](https://github.com/amish1706)

