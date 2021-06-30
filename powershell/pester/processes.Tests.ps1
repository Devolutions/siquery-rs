Import-Module "$PSScriptRoot/../siquery"

Describe 'processes table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$processes = $allSchemas | Where-Object { $_.BaseName -eq 'processes' }
				$processes | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$processes = $allSchemas | Where-Object { $_.BaseName -eq 'processes' }
				$schema = Get-Content $processes.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$processes = Get-Siq 'processes' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'processes' }).FullName -Raw

				$json = $processes | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
