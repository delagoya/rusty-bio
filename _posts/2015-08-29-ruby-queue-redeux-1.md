---
layout: post
title:  "# Ruby Queue, Part Duex (1)"
date:  2015-08-20 14:55:57 -0400
categories:
- blog
- ruby
- bioinformatics
permalink: ruby-queue-redeux-1
description: Using Ruby and SQS to index thousands of VCF files in S3

---


A while back, I had written a [post](http://defsci.blogspot.com/2010/01/ruby-aws-easy-map-reduce.html) on using Ruby to create a simple system to process FASTA files using BLAT. That post relied on [CloudCrowd](https://github.com/documentcloud/cloud-crowd), an open-source parallel processing using a Map-Reduce style application. While simple enough, the system relies on a central server and RDMS to handle job submission and distribution, and communications with nodes. 

I this example, we will use [Amazon Simple Queue Service (SQS)](http://aws.amazon.com/sqs/) to coordinate the list of files that need to be processed by our worker nodes. We will also boostrap an EC2 instance using [instance user data](http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html#instancedata-add-user-data) to start our processes that check the SQS queue for work items. 

Since this is revamp of an older post, I'll use Ruby and the AWS Ruby SDK to interact with the systems.   

Our goal is to **index thousands of [VCF files](https://www.broadinstitute.org/igv/viewing_vcf_files) stored in S3**. The indexing will be done using [bcftools](https://samtools.github.io/bcftools/bcftools.html) and the index will be stored back into S3 using the originating key as the prefix for the index file. 

## Step 1: Create SQS queues

Here we will use the [AWS CLI](http://aws.amazon.com/cli/) to create the work and error queues that we will use later:


```sh

## Create the two queues
aws sqs create-queue --queue-name vcfindex
{
    "QueueUrl": "https://queue.amazonaws.com/XXXXXXXXXXXX/vcfindex"
}
aws sqs create-queue --queue-name vcfindex-error
{
    "QueueUrl": "https://queue.amazonaws.com/XXXXXXXXXXXX/vcfindex-error"
}

## Get the QueueArn for the error queue
aws sqs get-queue-attributes  --queue-url "https://queue.amazonaws.com/XXXXXXXXXXXX/vcfindex-error" --attribute-names QueueArn
{
    "Attributes": {
        "QueueArn": "arn:aws:sqs:us-east-1:XXXXXXXXXXXX:vcfindex-error"
    }
}

## Set the error queue as the dead letter queue for the primary one
aws sqs set-queue-attributes --queue-url   "https://queue.amazonaws.com/XXXXXXXXXXXX/vcfindex"   --attributes '{"RedrivePolicy": "{ \"deadLetterTargetArn\": \"arn:aws:sqs:us-east-1:XXXXXXXXXXXX:vcfindex-error\", \"maxReceiveCount\": 2}" }'

```



Take note of the two queue URLs above as we will need them for the later steps. If you forget them it is easy enough to get the queue url like so: 


```sh

aws sqs get-queue-url --queue-name vcfindex

```


In next installment, we will be 