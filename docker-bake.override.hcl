
variable "TAG" {
  default = "devel"
}

variable "DOCKER_ORGANIZATION" {
  default = "juztamau5"
}

target "state-server" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-state-server:${TAG}"]
}

target "dispatcher" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-dispatcher:${TAG}"]
}

target "indexer" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-indexer:${TAG}"]
}

target "inspect-server" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-inspect-server:${TAG}"]
}

target "reader" {
  tags = ["${DOCKER_ORGANIZATION}/query-server:${TAG}"]
}

target "hardhat" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-hardhat-0.7.0:${TAG}"]
}

target "cli" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-cli-0.7.0:${TAG}"]
}
