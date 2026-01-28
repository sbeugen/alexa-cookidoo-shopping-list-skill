import * as cdk from "aws-cdk-lib";
import * as logs from "aws-cdk-lib/aws-logs";
import * as lambda from "aws-cdk-lib/aws-lambda";
import { RustFunction } from "cargo-lambda-cdk";
import { Construct } from "constructs";
import * as path from "node:path";

export class AlexaSkillStack extends cdk.Stack {
  public readonly lambdaFunction: RustFunction;

  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Create CloudWatch Log Group with 7-day retention
    const logGroup = new logs.LogGroup(this, "AlexaSkillLogGroup", {
      logGroupName: "/aws/lambda/alexa-cookidoo-skill",
      retention: logs.RetentionDays.ONE_WEEK,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    // Create the Rust Lambda function using cargo-lambda-cdk
    this.lambdaFunction = new RustFunction(this, "AlexaCookidooSkillFunction", {
      manifestPath: path.join(__dirname, "..", "..", "skill", "Cargo.toml"),
      binaryName: "bootstrap",
      architecture: lambda.Architecture.ARM_64,
      memorySize: 256,
      timeout: cdk.Duration.seconds(30),
      environment: {
        COOKIDOO_EMAIL: process.env.COOKIDOO_EMAIL || "",
        COOKIDOO_PASSWORD: process.env.COOKIDOO_PASSWORD || "",
        COOKIDOO_CLIENT_ID: process.env.COOKIDOO_CLIENT_ID || "",
        COOKIDOO_CLIENT_SECRET: process.env.COOKIDOO_CLIENT_SECRET || "",
        RUST_LOG: "info",
      },
      logGroup: logGroup,
    });

    // Add permission for Alexa Skills Kit to invoke the Lambda function
    this.lambdaFunction.addPermission("AlexaSkillKitPermission", {
      principal: new cdk.aws_iam.ServicePrincipal(
        "alexa-appkit.amazon.com"
      ),
      action: "lambda:InvokeFunction",
      sourceAccount: undefined, // Alexa doesn't provide source account
    });

    // Stack Outputs
    new cdk.CfnOutput(this, "LambdaFunctionArn", {
      value: this.lambdaFunction.functionArn,
      description: "Lambda Function ARN for Alexa Skill Endpoint",
      exportName: "AlexaCookidooSkillLambdaArn",
    });

    new cdk.CfnOutput(this, "LambdaFunctionName", {
      value: this.lambdaFunction.functionName,
      description: "Lambda Function Name",
      exportName: "AlexaCookidooSkillLambdaName",
    });

    new cdk.CfnOutput(this, "LogGroupName", {
      value: logGroup.logGroupName,
      description: "CloudWatch Log Group Name",
      exportName: "AlexaCookidooSkillLogGroupName",
    });
  }
}
