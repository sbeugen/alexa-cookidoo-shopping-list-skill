#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { AlexaSkillStack } from "../lib/alexa-skill-stack";

const app = new cdk.App();

new AlexaSkillStack(app, "AlexaCookidooSkillStack", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: process.env.CDK_DEFAULT_REGION || "eu-central-1",
  },
  description: "Alexa Cookidoo Shopping List Skill Lambda Function",
});
