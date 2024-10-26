# main.tf

# Configuración del proveedor
provider "aws" {
  region = "us-west-2"
}

# Configuración de VPC
module "vpc" {
  source = "terraform-aws-modules/vpc/aws"
  name   = "keyrust-vpc"
  cidr   = "10.0.0.0/16"
  azs    = ["us-west-2a", "us-west-2b"]
  public_subnets  = ["10.0.1.0/24", "10.0.2.0/24"]
  private_subnets = ["10.0.3.0/24", "10.0.4.0/24"]
}

# Configuración del clúster de Kubernetes
module "eks" {
  source          = "terraform-aws-modules/eks/aws"
  cluster_name    = "keyrust-cluster"
  cluster_version = "1.21"
  subnets         = module.vpc.public_subnets
  vpc_id          = module.vpc.vpc_id
}

# Otros recursos, como RDS o IAM roles, pueden configurarse aquí.

