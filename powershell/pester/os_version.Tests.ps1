Import-Module "$PSScriptRoot/../siquery"

Describe 'os_version table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$os_version = $allSchemas | Where-Object { $_.BaseName -eq 'os_version' }
				$os_version | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$os_version = $allSchemas | Where-Object { $_.BaseName -eq 'os_version' }
				$schema = Get-Content $os_version.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$os_version = Get-Siq 'os_version' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'os_version' }).FullName -Raw

				$json = $os_version | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
