
variable "TAG" {
  default = "0.8.2"
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

target "server-manager-broker-proxy" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-server-manager-broker-proxy:${TAG}"]
}

target "reader" {
  tags = ["${DOCKER_ORGANIZATION}/query-server:${TAG}"]
}

target "hardhat" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-hardhat:${TAG}"]
}

target "cli" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-cli:${TAG}"]
}
